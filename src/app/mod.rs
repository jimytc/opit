use crossterm::event::{KeyCode, KeyEvent};

use crate::request::HttpResponse;

const PANE_CYCLE: [Pane; 4] = [
    Pane::EndpointList,
    Pane::RequestBuilder,
    Pane::AuthConfig,
    Pane::ResponseViewer,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    EndpointList,
    RequestBuilder,
    AuthConfig,
    ResponseViewer,
}

pub struct AppState {
    pub focused: Pane,
    pub selected_operation_index: usize,
    operation_count: usize,
    last_response: Option<HttpResponse>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            focused: Pane::EndpointList,
            selected_operation_index: 0,
            operation_count: 0,
            last_response: None,
        }
    }

    pub fn set_operation_count(&mut self, count: usize) {
        self.operation_count = count;
    }

    pub fn set_response(&mut self, response: HttpResponse) {
        self.last_response = Some(response);
    }

    pub fn response(&self) -> Option<&HttpResponse> {
        self.last_response.as_ref()
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::BackTab => self.cycle_focus(key.code),
            KeyCode::Down if self.focused == Pane::EndpointList => {
                let max_index = self.operation_count.saturating_sub(1);
                if self.selected_operation_index < max_index {
                    self.selected_operation_index += 1;
                }
            }
            KeyCode::Up if self.focused == Pane::EndpointList => {
                self.selected_operation_index = self.selected_operation_index.saturating_sub(1);
            }
            _ => {}
        }
    }

    fn cycle_focus(&mut self, code: KeyCode) {
        let current_index = PANE_CYCLE
            .iter()
            .position(|pane| *pane == self.focused)
            .expect("focused pane is always in PANE_CYCLE");

        self.focused = match code {
            KeyCode::Tab => PANE_CYCLE[(current_index + 1) % PANE_CYCLE.len()],
            KeyCode::BackTab => {
                PANE_CYCLE[(current_index + PANE_CYCLE.len() - 1) % PANE_CYCLE.len()]
            }
            _ => self.focused,
        };
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
