extern crate cc;

fn main() {
    if cfg!(feature = "test") {
        cc::Build::new()
            .file("src/bcndecode.c")
            .compile("libbcndecode.a");
    }
}
