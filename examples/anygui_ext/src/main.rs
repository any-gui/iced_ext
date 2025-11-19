use iced::widget::{center, center_x, checkbox, column, image, svg, ExtContainer, ext_rounded_box, Container, ext_bordered_box};
use iced::{Element, Fill, color, Shrink};
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

        let svg =
            ExtContainer::new(
                svg(handle)
                    .width(Shrink)
                    .height(Fill)
                    .style(|_theme, _status| svg::Style {
                        color: if self.apply_color_filter {
                            Some(color!(0x0000ff))
                        } else {
                            None
                        },
                    })
            ).padding(10).style(ext_bordered_box);

        let png = Container::new(
            iced::widget::image(handle_png)
            .width(Shrink)
            .height(Fill)
        ).padding(10).style(bordered_box);

        let apply_color_filter =
            checkbox("Apply a color filter", self.apply_color_filter)
                .on_toggle(Message::ToggleColorFilter);

        let apply_png_color_filter =
            checkbox("Apply a color filter for Png", self.apply_png_filter)
                .on_toggle(Message::TogglePngFilter);

        center(column![center_x(svg), center_x(apply_color_filter),center_x(png),center_x(apply_png_color_filter)].spacing(20))
            .padding(20)
            .into()
    }
}
