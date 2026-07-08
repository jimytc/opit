use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{List, ListItem};

use crate::spec::Operation;

pub fn widget(operations: &[Operation]) -> List<'static> {
    let items: Vec<ListItem> = operations
        .iter()
        .map(|operation| {
            let mut lines = vec![Line::from(format!("{} {}", operation.method, operation.path))];
            if let Some(summary) = &operation.summary {
                lines.push(Line::from(format!("  {summary}")));
            }
            ListItem::new(lines)
        })
        .collect();
    List::new(items).highlight_style(Style::default().add_modifier(Modifier::REVERSED))
}
