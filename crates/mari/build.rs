fn main() {
    tonic_build::configure()
        .compile(&["../../proto/mari.proto"], &["../../proto/"])
        .unwrap();
}
