//extern crate cpp_build;
extern crate gcc;

fn main() {
    // Build a Redis pseudo-library so that we have symbols that we can link
    // against while building Rust code.
    gcc::Build::new()
        .file("c/listpack.c")
        .file("c/listpack_ext.c")
        .include("c/")
        .compile("liblistpack.a");
}
