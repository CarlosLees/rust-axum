use prost_build::Config;

fn main() -> std::io::Result<()> {
    //这种方式生成的文件不能显式地看到
    // prost_build::compile_protos(&["person.proto"],&["."]).unwrap()

    //指定具体生成的路径
    Config::new().out_dir("src/pb")
    // .bytes(&["."]) //自定义数据的类型
    .btree_map(&["scores"])
    .type_attribute(".", "#[derive(serde::Serialize,serde::Deserialize)]")//添加自定义derive
    .field_attribute("data", "#[serde(skip_serializing_if = \"Vec::is_empty\")]")//自定义添加字段
    .compile_protos(&["person.proto"],&["."])
}