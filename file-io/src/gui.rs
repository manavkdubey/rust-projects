use druid::widget::{Button, Flex, TextBox};
use druid::{AppLauncher, Data, Lens, Widget, WidgetExt, WindowDesc};
use std::fs;
use std::io;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub input_text: String,
    pub path_column: String,
}

pub fn read_file(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}
pub fn write_file(path: &str, contents: &str) -> io::Result<()> {
    fs::write(path, contents)
}

pub fn build_ui() -> impl Widget<AppState> {
    // Text box for general input
    let input = TextBox::new()
        .with_placeholder("Enter text here...")
        .lens(AppState::input_text);

    // Text box for path column (you might want to integrate a file picker here for better UX)
    let path_input = TextBox::new()
        .with_placeholder("Enter file path here...")
        .lens(AppState::path_column);

    // Submit button
    let submit_button = Button::new("Submit").on_click(move |_ctx, data: &mut AppState, _env| {
        write_file(&data.path_column, &data.input_text);
    }); // Reset button
    let reset_button = Button::new("Reset").on_click(|_ctx, data: &mut AppState, _env| {
        data.input_text.clear();
        data.path_column.clear();
    });

    // Organize widgets vertically using Flex
    Flex::column()
        .with_child(input)
        .with_spacer(8.0)
        .with_child(path_input)
        .with_spacer(8.0)
        .with_child(submit_button)
        .with_spacer(8.0)
        .with_child(reset_button)
}
