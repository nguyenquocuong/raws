use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{List, ListItem, Paragraph, Widget},
};

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
