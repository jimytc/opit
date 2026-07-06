use ratatui::widgets::Paragraph;

use crate::request::HttpResponse;

pub fn widget(response: Option<&HttpResponse>) -> Paragraph<'static> {
    let Some(response) = response else {
        return Paragraph::new("No response yet");
    };

    let mut lines = vec![format!("Status: {}", response.status)];
    lines.extend(response.body.lines().map(str::to_string));

    Paragraph::new(lines.join("\n"))
}
