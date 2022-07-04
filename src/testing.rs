use fltk::{prelude::*, window, app, button, enums::{Color, FrameType, Event, CallbackTrigger}, input, frame};
use fltk_theme::{WidgetTheme, ThemeType, widget_themes};
use fltk_flex::Flex;


#[derive(Default)]
struct MyButton {
    btn: button::Button,
}

impl MyButton {
    pub fn new(x: i32, y: i32, w: i32, h: i32, label: &str) -> Self {
        let mut btn = button::Button::new(x, y, w, h, None).with_label(label);
        btn.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
        btn.handle(|b, ev| match ev {
            Event::Enter => {
                b.set_frame(widget_themes::OS_BUTTON_UP_FRAME);
                b.redraw();
                true
            },
            Event::Leave => {
                b.set_frame(widget_themes::OS_DEFAULT_BUTTON_UP_BOX);
                b.redraw();
                true
            }
            _ => false,
        });
        Self {
            btn
        }
    }

    pub fn default() -> Self {
        return Self::new(0, 0, 0, 0, "");
    }
}

fltk::widget_extends!(MyButton, button::Button, btn);

// fn main() {
//     let a = app::App::default();
//     let theme = WidgetTheme::new(ThemeType::Aero);
//     theme.apply();
//     let mut win = window::Window::new(100, 100, 400, 400, "Tutorial");
//     let mut flex = Flex::default().with_size(200, 200).center_of_parent().column();
//     let mut input = input::SecretInput::default();
//     input.set_trigger(CallbackTrigger::Changed);
//     let mut frame = frame::Frame::default();
//     let mut but = MyButton::default().with_label("Click");

//     flex.end();

    
//     win.end();
//     win.show();
//     input.set_callback({
//         let mut frame = frame.clone();
//         move |i| {
//         frame.set_label(&i.value());
//     }});
//     but.set_callback(move |_| {
//         frame.set_label(&input.value());
//     });
//     a.run().unwrap();
// }