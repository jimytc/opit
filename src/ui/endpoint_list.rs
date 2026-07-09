use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::{List, ListItem};

use crate::spec::Operation;

fn operation_group(operation: &Operation) -> &str {
    operation.tags.first().map(String::as_str).unwrap_or("Untagged")
}

pub fn filtered_operations<'a>(operations: &'a [Operation], filter: &str) -> Vec<&'a Operation> {
    let matching: Vec<&Operation> = if filter.is_empty() {
        operations.iter().collect()
    } else {
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
    };

    let mut group_order: Vec<&str> = Vec::new();
    let mut groups: Vec<Vec<&Operation>> = Vec::new();
    for operation in matching {
        let group = operation_group(operation);
        let position = match group_order.iter().position(|existing| *existing == group) {
            Some(position) => position,
            None => {
                group_order.push(group);
                groups.push(Vec::new());
                group_order.len() - 1
            }
        };
        groups[position].push(operation);
    }
    groups.into_iter().flatten().collect()
}

fn operation_lines(operation: &Operation) -> Vec<Line<'static>> {
    let mut lines = vec![Line::from(format!(
        "{} {}",
        operation.method, operation.path
    ))];
    if let Some(summary) = &operation.summary {
        lines.push(Line::from(format!("  {summary}")));
    }
    lines
}

pub fn render(filtered: &[&Operation], selected_operation_index: usize) -> (List<'static>, usize) {
    let mut items = Vec::new();
    let mut visual_index = 0usize;
    let mut current_group: Option<&str> = None;

    for (index, operation) in filtered.iter().enumerate() {
        let group = operation_group(operation);
        if current_group != Some(group) {
            let header_style = Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD);
            items.push(ListItem::new(Line::styled(group.to_string(), header_style)));
            current_group = Some(group);
        }
        if index == selected_operation_index {
            visual_index = items.len();
        }
        items.push(ListItem::new(operation_lines(operation)));
    }

    (
        List::new(items).highlight_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::REVERSED),
        ),
        visual_index,
    )
}
