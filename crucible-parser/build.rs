// Build script to include Tree-Sitter parser in compilation
// Phase 2: Parser & Basic Verification - v0.2.0-alpha

fn main() {
    // Tell Cargo to rerun this build script if the grammar changes
    println!("cargo:rerun-if-changed=grammar.js");
    println!("cargo:rerun-if-changed=src/");
}
