use crossterm::event::{KeyCode, KeyEvent};

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
}

impl AppState {
    pub fn new() -> Self {
        Self {
            focused: Pane::EndpointList,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        let current_index = PANE_CYCLE
            .iter()
            .position(|pane| *pane == self.focused)
            .expect("focused pane is always in PANE_CYCLE");

        self.focused = match key.code {
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
