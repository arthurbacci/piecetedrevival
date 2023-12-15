fn main() {
    cc::Build::new()
        .include("src")
        .file("src/io.c")
        .compile("io");
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/io.c");
}

