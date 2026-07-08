use crossterm::event::{KeyCode, KeyEvent};

use crate::request::HttpResponse;

mod pane_editor;

pub use pane_editor::PaneEditor;

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
    pub selected_server_index: usize,
    pub request_builder: PaneEditor,
    pub auth_config: PaneEditor,
    operation_count: usize,
    server_count: usize,
    last_response: Option<HttpResponse>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            focused: Pane::EndpointList,
            selected_operation_index: 0,
            selected_server_index: 0,
            request_builder: PaneEditor::new(),
            auth_config: PaneEditor::new(),
            operation_count: 0,
            server_count: 0,
            last_response: None,
        }
    }

    pub fn set_operation_count(&mut self, count: usize) {
        self.operation_count = count;
    }

    pub fn set_server_count(&mut self, count: usize) {
        self.server_count = count;
    }

    pub fn set_response(&mut self, response: HttpResponse) {
        self.last_response = Some(response);
    }

    pub fn response(&self) -> Option<&HttpResponse> {
        self.last_response.as_ref()
    }

    pub fn is_editing(&self) -> bool {
        match self.focused {
            Pane::RequestBuilder => self.request_builder.editing_buffer().is_some(),
            Pane::AuthConfig => self.auth_config.editing_buffer().is_some(),
            Pane::EndpointList | Pane::ResponseViewer => false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::BackTab if !self.is_editing() => self.cycle_focus(key.code),
            _ => match self.focused {
                Pane::EndpointList => self.handle_endpoint_list_key(key.code),
                Pane::RequestBuilder => Self::handle_editor_key(&mut self.request_builder, key.code),
                Pane::AuthConfig => Self::handle_editor_key(&mut self.auth_config, key.code),
                Pane::ResponseViewer => {}
            },
        }
    }

    fn handle_endpoint_list_key(&mut self, code: KeyCode) {
        let before = self.selected_operation_index;
        match code {
            KeyCode::Down => {
                let max_index = self.operation_count.saturating_sub(1);
                if self.selected_operation_index < max_index {
                    self.selected_operation_index += 1;
                }
            }
            KeyCode::Up => {
                self.selected_operation_index = self.selected_operation_index.saturating_sub(1);
            }
            KeyCode::Char('s') => {
                if self.server_count > 0 {
                    self.selected_server_index =
                        (self.selected_server_index + 1) % self.server_count;
                }
            }
            _ => {}
        }
        if self.selected_operation_index != before {
            self.request_builder.reset();
        }
    }

    fn handle_editor_key(editor: &mut PaneEditor, code: KeyCode) {
        match code {
            KeyCode::Up => editor.move_up(),
            KeyCode::Down => editor.move_down(),
            KeyCode::Enter => {
                if editor.editing_buffer().is_some() {
                    editor.commit();
                } else {
                    editor.start_editing();
                }
            }
            KeyCode::Esc => editor.cancel(),
            KeyCode::Backspace => editor.pop_char(),
            KeyCode::Char(c) => editor.push_char(c),
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
