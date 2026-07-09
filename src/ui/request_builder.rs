use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::{List, ListItem};

use crate::app::RequestBuilderTab;
use crate::spec::Operation;

enum Row<'a> {
    Parameter(&'a crate::spec::Parameter),
    Custom { name: &'a str, location: &'a str },
    Body { example: Option<&'a str>, committed: bool },
    AddPlaceholder(&'static str),
}

impl Row<'_> {
    fn label(&self) -> &str {
        match self {
            Row::Parameter(parameter) => &parameter.name,
            Row::Custom { name, .. } => name,
            Row::Body { .. } => "Body",
            Row::AddPlaceholder(label) => label,
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
            Row::Custom { name, location } => format!("{name} ({location}, custom)"),
            Row::Body {
                example: Some(example),
                committed: false,
            } => {
                let compact_example = example.split_whitespace().collect::<Vec<_>>().join(" ");
                format!("Body — e.g. {compact_example}")
            }
            Row::Body { .. } => "Body".to_string(),
            Row::AddPlaceholder(label) => label.to_string(),
        }
    }

    fn editing_display(&self, buffer: &str) -> String {
        match self {
            Row::AddPlaceholder(_) => format!("+ {buffer}"),
            _ => format!("{}: {}", self.label(), buffer),
        }
    }
}

pub fn widget(
    tab: RequestBuilderTab,
    operation: Option<&Operation>,
    custom_headers: &[String],
    custom_query_params: &[String],
    selected_row: usize,
    editing: Option<&str>,
    body_committed: bool,
) -> List<'static> {
    let Some(operation) = operation else {
        return List::new(vec![ListItem::new("No operation selected")]);
    };

    let rows: Vec<Row> = match tab {
        RequestBuilderTab::Header => {
            let mut rows: Vec<Row> = operation
                .header_parameters()
                .into_iter()
                .map(Row::Parameter)
                .collect();
            rows.extend(custom_headers.iter().map(|name| Row::Custom {
                name,
                location: "header",
            }));
            rows.push(Row::AddPlaceholder("+ Add Header"));
            rows
        }
        RequestBuilderTab::Parameters => {
            let mut rows: Vec<Row> = operation
                .non_header_parameters()
                .into_iter()
                .map(Row::Parameter)
                .collect();
            rows.extend(custom_query_params.iter().map(|name| Row::Custom {
                name,
                location: "query",
            }));
            rows.push(Row::AddPlaceholder("+ Add Parameter"));
            rows
        }
        RequestBuilderTab::Payload => vec![Row::Body {
            example: operation.request_body_example.as_deref(),
            committed: body_committed,
        }],
    };

    let highlight_style = Style::default()
        .fg(Color::Green)
        .add_modifier(Modifier::REVERSED);
    let items: Vec<ListItem> = rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let text = if index == selected_row {
                match editing {
                    Some(buffer) => row.editing_display(buffer),
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
