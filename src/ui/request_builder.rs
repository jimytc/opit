use ratatui::style::{Modifier, Style};
use ratatui::widgets::{List, ListItem};

use crate::spec::Operation;

enum Row<'a> {
    Parameter(&'a crate::spec::Parameter),
    Body,
}

impl Row<'_> {
    fn label(&self) -> &str {
        match self {
            Row::Parameter(parameter) => &parameter.name,
            Row::Body => "Body",
        }
    }

    fn static_text(&self) -> String {
        match self {
            Row::Parameter(parameter) => {
                let requiredness = if parameter.required {
                    "required"
                } else {
                    "optional"
                };
                format!(
                    "{} ({}, {})",
                    parameter.name, parameter.location, requiredness
                )
            }
            Row::Body => "Body".to_string(),
        }
    }
}

pub fn widget(operation: Option<&Operation>, selected_row: usize, editing: Option<&str>) -> List<'static> {
    let Some(operation) = operation else {
        return List::new(vec![ListItem::new("No operation selected")]);
    };

    let mut rows: Vec<Row> = operation.parameters.iter().map(Row::Parameter).collect();
    if operation.has_request_body {
        rows.push(Row::Body);
    }

    if rows.is_empty() {
        return List::new(vec![ListItem::new("No parameters")]);
    }

    let highlight_style = Style::default().add_modifier(Modifier::REVERSED);
    let items: Vec<ListItem> = rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let text = if index == selected_row {
                match editing {
                    Some(buffer) => format!("{}: {}", row.label(), buffer),
                    None => row.static_text(),
                }
            } else {
                row.static_text()
            };
            let item = ListItem::new(text);
            if index == selected_row {
                item.style(highlight_style)
            } else {
                item
            }
        })
        .collect();
    List::new(items)
}
