use iced::{executor, Application, Color, Command, Settings};
use painting::Canvas;

mod css;
mod dom;
mod html;
mod layout;
mod painting;
mod style;

struct GUI {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

struct GUIProps {
    canvas: Canvas,
}

impl Application for GUI {
    type Executor = executor::Default;

    type Message = ();

    type Flags = GUIProps;

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        let gui = GUI {
            pixels: flags
                .canvas
                .pixels
                .iter()
                .map(|color| Color {
                    r: color.r as f32,
                    g: color.g as f32,
                    b: color.b as f32,
                    a: color.a as f32,
                })
                .collect(),
            width: flags.canvas.width,
            height: flags.canvas.height,
        };
        (gui, Command::none())
    }

    fn title(&self) -> String {
        "Robinson".to_string()
    }

    fn update(
        &mut self,
        _message: Self::Message,
        _clipboard: &mut iced::Clipboard,
    ) -> iced::Command<Self::Message> {
        Command::none()
    }

    fn view(&mut self) -> iced::Element<'_, Self::Message> {
        todo!()
    }
}

fn main() -> iced::Result {
    GUI::run(Settings::with_flags(GUIProps {
        canvas: Canvas {
            pixels: vec![],
            width: 800,
            height: 800,
        },
    }))
}
