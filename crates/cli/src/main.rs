fn main() {
    if let Err(error) = dellctl::run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}
