use ratatui::style::{Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{List, ListItem};

use crate::spec::Operation;

pub fn filtered_operations<'a>(operations: &'a [Operation], filter: &str) -> Vec<&'a Operation> {
    if filter.is_empty() {
        return operations.iter().collect();
    }
    let filter = filter.to_lowercase();
    operations
        .iter()
        .filter(|operation| {
            format!(
                "{} {} {}",
                operation.method,
                operation.path,
                operation.summary.as_deref().unwrap_or("")
            )
            .to_lowercase()
            .contains(&filter)
        })
        .collect()
}

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
