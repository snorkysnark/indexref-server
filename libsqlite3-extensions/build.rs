fn main() {
    println!("cargo:rerun-if-changed=sqlite3/ext/closure.c");
    println!("cargo:rerun-if-changed=sqlite3/include/sqlite3.h");
    println!("cargo:rerun-if-changed=sqlite3/include/sqlite3ext.h");

    cc::Build::new()
        .include("sqlite3/include")
        .file("sqlite3/ext/closure.c")
        .compile("sqlite3-ext");
}
