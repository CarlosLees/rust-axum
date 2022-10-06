use prost::Message;
use rust_axum::pb::*;

fn main() {
    let phone_number = vec![PhoneNumber::new("1111",PhoneType::Home),
    PhoneNumber::new("22222",PhoneType::Work)];

    let person = Person::new("张三",1,"101.com",phone_number);

    let v1 = person.encode_to_vec();

    let person1 = Person::decode(v1.as_ref()).unwrap();

    println!("{:?}",person);
    println!("{:?}",person1);
}