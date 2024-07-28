use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, List, ListItem, Paragraph, Widget},
};

#[derive(Default)]
pub struct ContextWidget {}

impl ContextWidget {}

impl Widget for ContextWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        Paragraph::new("IAM ARN:")
            //.block(
            //    Block::bordered().title(" Context "),
            //    //.title_alignment(Alignment::Center),
            //)
            .render(area, buf);
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
        let outer_block = Block::bordered().title("Outer");
        let left_inner_block = Block::bordered().title("Left");
        let right_inner_block = Block::bordered().title("Right");

        let [keybinding_area, description_area] =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)])
                .areas(outer_area);

        let keybinding_items: Vec<ListItem> = self
            .keybindings
            .iter()
            .map(|item| ListItem::new(Line::blue(item.binding.into())))
            .collect();
        let description_items: Vec<ListItem> = self
            .keybindings
            .iter()
            .map(|item| ListItem::new(Line::from(item.description)))
            .collect();

        //outer_block.render(outer_area, buf);
        List::new(keybinding_items)
            //.block(left_inner_block)
            .render(keybinding_area, buf);
        List::new(description_items)
            //.block(right_inner_block)
            .render(description_area, buf);
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

pub struct ContentWidget {
    title: String,
}

impl Default for ContentWidget {
    fn default() -> Self {
        ContentWidget {
            title: "Clusters".to_string(),
        }
    }
}

impl ContentWidget {}

impl Widget for ContentWidget {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        //List::new([])
        //.block(
        Block::bordered()
            .title(format!(" {} ", self.title))
            .title_alignment(Alignment::Center)
            //)
            .render(area, buf);
    }
}
