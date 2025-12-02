use std::path::PathBuf;

fn main() {
    let grammar_dir: PathBuf = ["tree-sitter-http"].iter().collect();

    cc::Build::new()
        .include(&grammar_dir)
        .file(grammar_dir.join("parser.c"))
        .warnings(false)
        .compile("tree-sitter-http");

    println!("cargo:rerun-if-changed=tree-sitter-http/parser.c");
}
