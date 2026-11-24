// build.rs serves to generate Rust code from components like .slint.
fn main() {
    if let Err(e) = slint_build::compile("src/ui/main.slint") {
        eprintln!("Slint compilation has failed: {}", e);
        std::process::exit(1);
    }
}
