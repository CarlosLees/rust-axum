use async_trait::async_trait;
use axum::extract::{FromRequest, RequestParts, TypedHeader};
use axum::http::StatusCode;
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, post};
use axum::{headers::authorization::Bearer, headers::Authorization, Json, Router, Server};
use jsonwebtoken as jwt;
use jsonwebtoken::Validation;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

const SECRET: &[u8] = b"beef";

#[derive(Deserialize, Serialize, Debug)]
struct Index {
    id: usize,
    title: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct IndexPost {
    title: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    id: usize,
    username: String,
    exp: usize,
}

#[async_trait]
impl<B> FromRequest<B> for Claims
where
    B: Send,
{
    type Rejection = HttpError;

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request(req)
                .await
                .map_err(|e| HttpError::Auth)
                .unwrap();
        let key = jwt::DecodingKey::from_secret(SECRET);
        let token = jwt::decode(bearer.token(), &key, &Validation::default())
            .map_err(|_| HttpError::Auth)
            .unwrap();
        Ok(token.claims)
    }
}

#[derive(Debug)]
enum HttpError {
    Auth,
    Internal,
}

impl IntoResponse for HttpError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            HttpError::Auth => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED"),
            HttpError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER_ERROR"),
        };
        (code, msg).into_response()
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handle_index))
        .route("/info", get(get_index).post(post_index))
        .route("/login", post(login_handler))
        .route("/hello", post(hello_handler));
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_index() -> Html<&'static str> {
    Html("Hello World")
}

async fn hello_handler() -> Html<&'static str> {
    Html("Hello Handler")
}

async fn get_index() -> Json<Vec<Index>> {
    Json(vec![
        Index {
            id: 0,
            title: "json_1".to_string(),
            content: "json_content_1".to_string(),
        },
        Index {
            id: 1,
            title: "json_2".to_string(),
            content: "json_content_2".to_string(),
        },
    ])
}

async fn post_index(claims: Claims, Json(todo): Json<IndexPost>) -> Json<Claims> {
    println!("{:?}", todo.title);
    StatusCode::CREATED;
    Json(claims)
}

async fn login_handler(Json(login): Json<LoginRequest>) -> Json<LoginResponse> {
    //生成token后再返回
    let claims = Claims {
        id: 1,
        username: login.username,
        exp: get_epoch() + 14 * 24 * 60 * 60,
    };
    let key = jwt::EncodingKey::from_secret(SECRET);
    let token = jwt::encode(&jwt::Header::default(), &claims, &key).unwrap();
    Json(LoginResponse { token })
}

fn get_epoch() -> usize {
    use std::time::SystemTime;
    SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize
}
