mod widgets;

use widgets::{Button, Label, Widget, Window};

fn main() {
    let mut window: Window = Window::new("Rust GUI Demo 1.23");
    window.add_widget(Box::new(Label::new("This is small GUI demo")));
    window.add_widget(Box::new(Button::new("Click me")));
    window.draw()
}
