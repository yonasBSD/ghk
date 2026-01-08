// Build script for ghk - generates shell completions during build
fn main() {
    // Shell completions are generated at runtime via `ghk completions <shell>`
    // This build.rs is kept minimal for now
    println!("cargo:rerun-if-changed=src/cli.rs");
}
