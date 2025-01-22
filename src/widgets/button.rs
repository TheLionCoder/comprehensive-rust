use super::{label::Label, Widget};

pub struct Button {
    label: Label,
}

impl Button {
    pub(crate) fn new(label: &str) -> Button {
        Button {
            label: Label::new(label),
        }
    }
}

impl Widget for Button {
    fn width(&self) -> usize {
        self.label.width() + 8 // a bit of padding
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let width: usize = self.width();
        let mut label: String = String::new();
        self.label.draw_into(&mut label);

        writeln!(buffer, "+{:-<width$}+", "").unwrap();
        for line in label.lines() {
            writeln!(buffer, "|{:^width$}|", &line).unwrap();
        }
        writeln!(buffer, "+{:-<width$}+", "").unwrap();
    }
}
