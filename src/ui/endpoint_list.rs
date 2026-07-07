use ratatui::style::{Modifier, Style};
use ratatui::widgets::{List, ListItem};

use crate::spec::Operation;

pub fn widget(operations: &[Operation]) -> List<'_> {
    let items: Vec<ListItem> = operations
        .iter()
        .map(|operation| ListItem::new(format!("{} {}", operation.method, operation.path)))
        .collect();
    List::new(items).highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}
