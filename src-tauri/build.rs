fn main() {
    prost_build::compile_protos(&["src/proto/index.proto"], &["src/proto/"]).unwrap();
    tauri_build::build()
}
