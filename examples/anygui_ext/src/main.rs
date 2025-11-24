use iced::widget::{center, center_x, checkbox, column, image, svg, ExtContainer, ExtBorder, ExtBoxShadow, ExtShadow, Container, ext_bordered_box, row, Style, ExtPath};
use iced::{Element, Fill, color, Shrink, Vector, Color, Theme};
use iced::border::Radius;
use iced::widget::container::{bordered_box, rounded_box};

pub fn main() -> iced::Result {
    iced::run(Tiger::update, Tiger::view)
}

#[derive(Debug, Default)]
struct Tiger {
    apply_color_filter: bool,
    apply_png_filter: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    ToggleColorFilter(bool),
    TogglePngFilter(bool),
}

impl Tiger {
    fn update(&mut self, message: Message) {
        match message {
            Message::ToggleColorFilter(apply_color_filter) => {
                self.apply_color_filter = apply_color_filter;
            }
            Message::TogglePngFilter(png) => {
                self.apply_png_filter = png;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        let handle = svg::Handle::from_path(format!(
            "{}/resources/tiger.svg",
            env!("CARGO_MANIFEST_DIR")
        ));

        let handle_png = image::Handle::from_path(format!(
            "{}/resources/tiger.png",
            env!("CARGO_MANIFEST_DIR")
        ));

        let padding = 30;
        let spacing = 100;

        // Opacity And Shadow with spread
        let svg =
            ExtContainer::new(
                svg(handle.clone())
                    .width(Shrink)
                    .height(Fill)
                    .style(|_theme: &Theme, _status| svg::Style {
                        color: if self.apply_color_filter {
                            Some(color!(0x0000ff))
                        } else {
                            None
                        },
                    })
            ).opacity(0.3).padding(padding).style(|theme| {
                let palette = theme.extended_palette();
                Style {
                    background: Some(palette.background.weakest.color.into()),
                    text_color: Some(palette.background.weakest.text),
                    border: ExtBorder::from_color(palette.background.weak.color),
                    shadow: ExtShadow {
                        shadows: vec![ExtBoxShadow{
                            color: Color::BLACK.scale_alpha(0.5),
                            offset: Vector::new(20.0,10.),
                            blur_radius: 5.,
                            spread: 10.,
                            is_inset: false,
                        }]
                    },
                    path: ExtPath::Quad(Radius::new(4.)),
                    snap: false,
                }
            });
        // Opacity And shadow No Spread
        let svg2 =
            ExtContainer::new(
                iced::widget::svg(handle)
                    .width(Shrink)
                    .height(Fill)
                    .style(|_theme: &Theme, _status| svg::Style {
                        color: if self.apply_color_filter {
                            Some(color!(0x0000ff))
                        } else {
                            None
                        },
                    })
            ).padding(padding).style(|theme| {
                let palette = theme.extended_palette();
                Style {
                    background: Some(palette.background.weakest.color.into()),
                    text_color: Some(palette.background.weakest.text),
                    border: ExtBorder::from_color(palette.background.weak.color),
                    shadow: ExtShadow {
                        shadows: vec![ExtBoxShadow{
                            color: Color::BLACK.scale_alpha(0.5),
                            offset: Vector::new(20.0,10.),
                            blur_radius: 5.,
                            spread: 0.,
                            is_inset: false,
                        }]
                    },
                    path: ExtPath::Quad(Radius::new(4.)),
                    snap: false,
                }
            });
        // No Opacity And Shadow Top Right
        let png = ExtContainer::new(
            iced::widget::image(handle_png.clone())
            .width(Shrink)
            .height(Fill)
        ).padding(padding).style(|theme:&Theme| {
            let palette = theme.extended_palette();
            Style {
                background: Some(palette.background.weakest.color.into()),
                text_color: Some(palette.background.weakest.text),
                border: ExtBorder::from_color(palette.background.weak.color),
                shadow: ExtShadow {
                    shadows: vec![ExtBoxShadow{
                        color: Color::BLACK.scale_alpha(0.5),
                        offset: Vector::new(-20.0,-10.),
                        blur_radius: 5.,
                        spread: 10.,
                        is_inset: false,
                    }]
                },
                path: ExtPath::Quad(Radius::new(4.)),
                snap: false,
            }
        });

        // No Opacity And No Shadow
        let png2 = Container::new(
            iced::widget::image(handle_png)
                .width(Shrink)
                .height(Fill)
        ).padding(padding).style(bordered_box);

        // let apply_color_filter =
        //     checkbox("Apply a color filter", self.apply_color_filter)
        //         .on_toggle(Message::ToggleColorFilter);
        //
        // let apply_png_color_filter =
        //     checkbox("Apply a color filter for Png", self.apply_png_filter)
        //         .on_toggle(Message::TogglePngFilter);

        ExtContainer::new(
            center(column![row![center_x(svg),center_x(svg2)],row![center_x(png),center_x(png2)]].spacing(spacing))
                .padding(100)
        ).width(Fill).height(Fill).style(|theme|{
            Style::default().background(Color::from_rgb8(180,180,180))
        }).into()

    }
}
