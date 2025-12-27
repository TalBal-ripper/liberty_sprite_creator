#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
mod gui;
mod image;

fn main() {
    if let Err(e) = gui::app::run_gui() {
        eprintln!("Error running GUI: {}", e);
    }
}
