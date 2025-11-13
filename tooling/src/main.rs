fn main() {
    // Minimal binary to satisfy workspace checks without warnings.
    // Uses no stdout unless explicitly requested via env to avoid lint noise.
    if std::env::var_os("TOOLING_VERBOSE").is_some() {
        println!("tooling ready");
    }
}
