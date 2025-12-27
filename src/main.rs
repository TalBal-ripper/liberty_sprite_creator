mod gui;
mod image;

fn main() {
    if let Err(e) = gui::app::run_gui() {
        eprintln!("Error running GUI: {}", e);
    }
}
