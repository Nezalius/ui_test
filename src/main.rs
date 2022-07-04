use fltk::{prelude::*, window, app, button, enums::{Color, FrameType, Event, CallbackTrigger}, input, frame};
use fltk_theme::{WidgetTheme, ThemeType, widget_themes};
use fltk_flex::Flex;

#[derive(Clone)]
struct Talent {
    name: String,
    value: String,
}


impl Talent {
    pub fn new(name: String, val: String) -> Self {
        Self {name: name, value: val}
    }

    pub fn print(&self) {
        println!("Das Talent {} hat den Wert: {}", self.name, self.value);
    }
}

struct Line {
    btn: button::Button,
    value_field: input::Input,
    talent: Talent,
    x: i32,
    y: i32,
}

impl Line {
    pub fn new(x: i32, y: i32, talent: Talent) -> Self {
        let mut btn = button::Button::default().with_size(150, 100).with_pos(x, y).with_label(&talent.name);
        let talent_clone: Talent = talent.clone();
        btn.set_callback(move |b| {
            talent_clone.print();
        });
        let mut input = input::Input::default().with_size(150, 100).with_pos(x + 151, y);
        input.set_value(&talent.value);
        Self {
            btn: btn,
            value_field: input,
            talent: talent,
            x: x,
            y: y,
        }
    }
}

struct Column {
    talents: Vec<Line>,
    x: i32,
    y: i32,
}

impl Column {
    pub fn create_empty() -> Self {
        Self {
            talents: vec![],
            x: 110,
            y: 110,
        }
    }

    pub fn add_row(&mut self, talent: Talent) {
        let mut new_line = Line::new(self.x, 
                                     self.y + 100*self.talents.len() as i32,
                                     talent);
        self.talents.push(new_line);
    }
}

fn main() {
    let a = app::App::default();
    let theme = WidgetTheme::new(ThemeType::HighContrast);
    theme.apply();
    let mut win = window::Window::new(100, 100, 400, 400, "Testing");
    let mut zweihand: Talent = Talent::new("Zweihand".to_string(), "100".to_string());
    let mut athletik: Talent = Talent::new("Athletik".to_string(), "90".to_string());
    let mut body: Column = Column::create_empty();
    body.add_row(zweihand);
    body.add_row(athletik);

    win.end();
    win.show();
    a.run().unwrap();

}
