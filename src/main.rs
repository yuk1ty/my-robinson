use iced::{
    canvas::{Frame, Path, Program},
    executor, Application, Color, Command, Length, Point, Settings, Size,
};

use crate::painting::Canvas;

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

struct RenderingCanvas {
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
                .map(|color| Color::from_rgba8(color.r, color.g, color.b, color.a as f32))
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
        iced::Canvas::new(RenderingCanvas {
            pixels: vec![Color::BLACK, Color::BLACK, Color::BLACK],
            width: 3,
            height: 1,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

impl Program<()> for RenderingCanvas {
    fn draw(
        &self,
        _bounds: iced::Rectangle,
        _cursor: iced::canvas::Cursor,
    ) -> Vec<iced::canvas::Geometry> {
        let mut frame = Frame::new(Size::new(self.width as f32, self.height as f32));

        self.pixels.iter().for_each(|pixel| {
            let px = Path::rectangle(
                Point::new(0.0, 0.0),
                iced::Size {
                    width: 1.0,
                    height: 1.0,
                },
            );

            frame.fill(&px, *pixel);
        });

        vec![frame.into_geometry()]
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
