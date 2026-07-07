use ratatui::widgets::{List, ListItem};

use crate::spec::{SecurityScheme, SecuritySchemeKind};

pub fn widget(schemes: &[SecurityScheme]) -> List<'static> {
    if schemes.is_empty() {
        return List::new(vec![ListItem::new("No security schemes defined")]);
    }

    let items: Vec<ListItem> = schemes
        .iter()
        .map(|scheme| {
            let description = match &scheme.kind {
                SecuritySchemeKind::ApiKey { location, .. } => {
                    format!("apiKey ({location})")
                }
                SecuritySchemeKind::Http { scheme } => format!("http ({scheme})"),
                SecuritySchemeKind::OAuth2 => "oauth2".to_string(),
                SecuritySchemeKind::OpenIdConnect => "openIdConnect".to_string(),
            };
            ListItem::new(format!("{}: {}", scheme.name, description))
        })
        .collect();
    List::new(items)
}
