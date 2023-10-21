mod gui;
extern crate druid;
use druid::{AppLauncher, WindowDesc};
use std::sync::{Arc, Mutex};

fn main() {
    let main_window = WindowDesc::new(gui::build_ui()).title("File i/o"); // Set the title for your application window

    // Define the initial  state for your app. This might vary based on your AppState struct definition.
    let initial_state = gui::AppState {
        input_text: "".to_string(),
        path_column: "/Users/manavkumardubey/Desktop/projects/rust/rust-projects".to_string(),
    };

    // Launch the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Unable to launch application")
}
