// Hack to get intellisense working for `post_build.rs`.
#[cfg(test)] // Always false in build scripts
#[path = "post_build.rs"]
mod __post_build;

fn main() {
    // post_build::metabuild();
}
