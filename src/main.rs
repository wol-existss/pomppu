// Import generated code from slint
slint::include_modules!();
fn main() -> Result<(), slint::PlatformError> {
    // Create MainWindow instance
    let ui = MainWindow::new()?;
    ui.run()?;
    Ok(())
}