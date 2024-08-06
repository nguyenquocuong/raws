use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{List, ListItem, Paragraph, Widget},
    Frame,
};

use crate::components::Component;

#[derive(Default)]
pub struct Context {
    iam_arn: String,
}

impl Context {
    pub fn iam_arn(mut self, arn: String) -> Self {
        self.iam_arn = arn;
        self
    }
}

impl Component for Context {
    fn draw(&mut self, frame: &mut Frame, rect: Rect) {
        let outer_area = rect;

        let [title_area, value_area] =
            Layout::horizontal([Constraint::Length(9), Constraint::Percentage(100)])
                .areas(outer_area);

        let title_items: Vec<ListItem> = vec![
            ListItem::new(Line::from("IAM ARN: ")),
            ListItem::new(Line::from("Cluster: ")),
        ];
        let value_items: Vec<ListItem> =
            vec![ListItem::new(Line::yellow(self.iam_arn.clone().into()))];

        frame.render_widget(List::new(title_items), title_area);
        frame.render_widget(List::new(value_items), value_area);
    }
}

struct KeybindingItem<'a> {
    binding: &'a str,
    description: &'a str,
}

pub struct KeybindingsWidget<'a> {
    keybindings: Vec<KeybindingItem<'a>>,
}

impl<'a> Default for KeybindingsWidget<'a> {
    fn default() -> Self {
        KeybindingsWidget {
            keybindings: vec![
                KeybindingItem {
                    binding: "<q>",
                    description: "Quit",
                },
                //KeybindingItem {
                //    binding: "<j>",
                //    description: "Move down",
                //},
                //KeybindingItem {
                //    binding: "<k>",
                //    description: "Move up",
                //},
            ],
        }
    }
}

impl<'a> Widget for KeybindingsWidget<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let outer_area = area;

        let [keybinding_area, description_area] =
            Layout::horizontal([Constraint::Length(4), Constraint::Percentage(100)])
                .areas(outer_area);

        let keybinding_items: Vec<ListItem> = self
            .keybindings
            .iter()
            .map(|item| ListItem::new(Line::blue(item.binding.into())))
            .collect();
        let description_items: Vec<ListItem> = self
            .keybindings
            .iter()
            .map(|item| ListItem::new(Line::dark_gray(item.description.into())))
            .collect();

        List::new(keybinding_items).render(keybinding_area, buf);
        List::new(description_items).render(description_area, buf);
    }
}

#[derive(Default)]
pub struct LogoWidget {}

impl LogoWidget {}

impl Widget for LogoWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("")
            //.block(Block::bordered().title(" LOGO "))
            .render(area, buf);
    }
}
