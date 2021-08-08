use std::{fs::File, io::Read};

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

struct RenderingCanvas<'a> {
    pixels: &'a Vec<Color>,
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
            pixels: &self.pixels,
            width: self.width,
            height: self.height,
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}

impl<'a> Program<()> for RenderingCanvas<'a> {
    fn draw(
        &self,
        _bounds: iced::Rectangle,
        _cursor: iced::canvas::Cursor,
    ) -> Vec<iced::canvas::Geometry> {
        let mut frame = Frame::new(Size::new(self.width as f32, self.height as f32));

        (1..self.width).for_each(|x| {
            (1..self.height).for_each(|y| {
                let px = Path::rectangle(
                    Point::new(x as f32, y as f32),
                    iced::Size {
                        width: 1.0,
                        height: 1.0,
                    },
                );

                let color = self.pixels[y * self.width + x];

                frame.fill(&px, color);
            })
        });

        vec![frame.into_geometry()]
    }
}

fn read_source(filename: String) -> String {
    let mut str = String::new();
    File::open(filename)
        .unwrap()
        .read_to_string(&mut str)
        .unwrap();
    str
}

fn main() -> iced::Result {
    let mut viewport: layout::Dimensions = Default::default();
    viewport.content.width = 800.0;
    viewport.content.height = 600.0;

    let html = read_source("test/test.html".to_string());
    let css = read_source("test/test.css".to_string());

    let root_node = html::parse(html);
    let stylesheet = css::parse(css);
    let style_root = style::style_tree(&root_node, &stylesheet);
    let layout_root = layout::layout_tree(&style_root, viewport);

    let canvas = painting::paint(&layout_root, viewport.content);

    GUI::run(Settings::with_flags(GUIProps {
        canvas: Canvas {
            pixels: canvas.pixels,
            width: viewport.content.width as usize,
            height: viewport.content.height as usize,
        },
    }))
}
