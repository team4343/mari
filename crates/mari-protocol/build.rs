fn main() {
    tonic_build::configure()
        .compile(&["./protocol/mari.proto"], &["./protocol/"])
        .unwrap();
}
