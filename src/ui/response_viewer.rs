use ratatui::widgets::{Paragraph, Wrap};

use crate::request::{pretty_print_if_json, HttpResponse};

pub fn widget(response: Option<&HttpResponse>) -> Paragraph<'static> {
    let Some(response) = response else {
        return Paragraph::new("No response yet");
    };

    let pretty_body = pretty_print_if_json(&response.body);
    let mut lines = vec![format!("Status: {}", response.status)];
    lines.extend(pretty_body.lines().map(str::to_string));

    Paragraph::new(lines.join("\n")).wrap(Wrap { trim: false })
}
