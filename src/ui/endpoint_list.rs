use ratatui::style::{Color, Modifier, Style};
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

fn operation_lines(operation: &Operation) -> Vec<Line<'static>> {
    let mut lines = vec![Line::from(format!("{} {}", operation.method, operation.path))];
    if let Some(summary) = &operation.summary {
        lines.push(Line::from(format!("  {summary}")));
    }
    lines
}

pub fn render(filtered: &[&Operation], selected_operation_index: usize) -> (List<'static>, usize) {
    let mut group_order: Vec<&str> = Vec::new();
    let mut groups: Vec<Vec<usize>> = Vec::new();

    for (index, operation) in filtered.iter().enumerate() {
        let group = operation
            .tags
            .first()
            .map(String::as_str)
            .unwrap_or("Untagged");
        let group_position = match group_order.iter().position(|existing| *existing == group) {
            Some(position) => position,
            None => {
                group_order.push(group);
                groups.push(Vec::new());
                group_order.len() - 1
            }
        };
        groups[group_position].push(index);
    }

    let mut items = Vec::new();
    let mut visual_index = 0usize;
    for (group_name, operation_indices) in group_order.iter().zip(groups.iter()) {
        items.push(ListItem::new(Line::from(group_name.to_string())));
        for &index in operation_indices {
            if index == selected_operation_index {
                visual_index = items.len();
            }
            items.push(ListItem::new(operation_lines(filtered[index])));
        }
    }

    (
        List::new(items)
            .highlight_style(Style::default().fg(Color::Green).add_modifier(Modifier::REVERSED)),
        visual_index,
    )
}
