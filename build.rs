fn main() {
    cc::Build::new()
        .cpp(true)
        .file("libbrp/src/huffman.cpp")
        .compile("libhuffman.a");
}
