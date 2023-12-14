fn main() {
    cc::Build::new()
        .include("src")
        .file("src/io.c")
        .compile("io")
}
