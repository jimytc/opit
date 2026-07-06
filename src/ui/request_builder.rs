use ratatui::widgets::{List, ListItem};

use crate::spec::Operation;

pub fn widget(operation: Option<&Operation>) -> List<'static> {
    let Some(operation) = operation else {
        return List::new(vec![ListItem::new("No operation selected")]);
    };

    if operation.parameters.is_empty() {
        return List::new(vec![ListItem::new("No parameters")]);
    }

    let items: Vec<ListItem> = operation
        .parameters
        .iter()
        .map(|parameter| {
            let requiredness = if parameter.required {
                "required"
            } else {
                "optional"
            };
            ListItem::new(format!(
                "{} ({}, {})",
                parameter.name, parameter.location, requiredness
            ))
        })
        .collect();
    List::new(items)
}
