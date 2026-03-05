slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    if std::env::var("WAYLAND_DISPLAY").is_ok() {
        std::env::set_var("QT_QPA_PLATFORM", "xcb");
    }
    let main_window = MainWindow::new()?;

    main_window.run()
}