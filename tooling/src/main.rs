fn main() {
    // Minimal binary to satisfy workspace checks without warnings.
    // No stdout/stderr to avoid disallowed macros (see clippy.toml).
    let _ = std::env::var_os("TOOLING_VERBOSE");
}
