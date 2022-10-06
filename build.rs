use prost_build::Config;

fn main() -> std::io::Result<()> {
    //这种方式生成的文件不能显式地看到
    // prost_build::compile_protos(&["person.proto"],&["."]).unwrap()

    //指定具体生成的路径
    Config::new().out_dir("src/pb").compile_protos(&["person.proto"],&["."])
}