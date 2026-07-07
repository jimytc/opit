use ratatui::style::{Modifier, Style};
use ratatui::widgets::{List, ListItem};

use crate::spec::{SecurityScheme, SecuritySchemeKind};

fn static_text(scheme: &SecurityScheme) -> String {
    let description = match &scheme.kind {
        SecuritySchemeKind::ApiKey { location, .. } => format!("apiKey ({location})"),
        SecuritySchemeKind::Http { scheme } if scheme == "basic" => {
            "http (basic) — enter as user:pass".to_string()
        }
        SecuritySchemeKind::Http { scheme } => format!("http ({scheme})"),
        SecuritySchemeKind::OAuth2 => "oauth2 (not editable yet)".to_string(),
        SecuritySchemeKind::OpenIdConnect => "openIdConnect (not editable yet)".to_string(),
    };
    format!("{}: {}", scheme.name, description)
}

pub fn widget(schemes: &[SecurityScheme], selected_row: usize, editing: Option<&str>) -> List<'static> {
    if schemes.is_empty() {
        return List::new(vec![ListItem::new("No security schemes defined")]);
    }

    let highlight_style = Style::default().add_modifier(Modifier::REVERSED);
    let items: Vec<ListItem> = schemes
        .iter()
        .enumerate()
        .map(|(index, scheme)| {
            let text = if index == selected_row {
                match editing {
                    Some(buffer) => format!("{}: {}", scheme.name, buffer),
                    None => static_text(scheme),
                }
            } else {
                static_text(scheme)
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
