use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::request::HttpResponse;

mod pane_editor;

pub use pane_editor::PaneEditor;

const PANE_CYCLE: [Pane; 5] = [
    Pane::EndpointList,
    Pane::AuthConfig,
    Pane::RequestBuilder,
    Pane::CurlPreview,
    Pane::ResponseViewer,
];

const REQUEST_BUILDER_TAB_CYCLE: [RequestBuilderTab; 3] = [
    RequestBuilderTab::Header,
    RequestBuilderTab::Parameters,
    RequestBuilderTab::Payload,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    EndpointList,
    RequestBuilder,
    AuthConfig,
    CurlPreview,
    ResponseViewer,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestBuilderTab {
    Header,
    Parameters,
    Payload,
}

#[derive(Default)]
pub struct RequestBuilderState {
    pub headers: PaneEditor,
    pub parameters: PaneEditor,
    pub payload: PaneEditor,
    pub custom_headers: Vec<String>,
    pub custom_query_params: Vec<String>,
}

impl RequestBuilderState {
    fn new() -> Self {
        Self::default()
    }

    fn reset(&mut self) {
        self.headers.reset();
        self.parameters.reset();
        self.payload.reset();
        self.custom_headers.clear();
        self.custom_query_params.clear();
    }
}

pub struct AppState {
    pub focused: Pane,
    pub selected_operation_index: usize,
    pub selected_server_index: usize,
    pub endpoint_filter: String,
    pub request_builder: RequestBuilderState,
    pub request_builder_tab: RequestBuilderTab,
    pub auth_config: PaneEditor,
    operation_count: usize,
    server_count: usize,
    filtering: bool,
    last_selected_identity: Option<(String, String)>,
    last_response: Option<HttpResponse>,
    curl_preview_scroll: u16,
    curl_preview_max_scroll: u16,
    response_viewer_scroll: u16,
    response_viewer_max_scroll: u16,
    header_param_names: Vec<String>,
    parameter_param_names: Vec<String>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            focused: Pane::EndpointList,
            selected_operation_index: 0,
            selected_server_index: 0,
            endpoint_filter: String::new(),
            request_builder: RequestBuilderState::new(),
            request_builder_tab: RequestBuilderTab::Header,
            auth_config: PaneEditor::new(),
            operation_count: 0,
            server_count: 0,
            filtering: false,
            last_selected_identity: None,
            last_response: None,
            curl_preview_scroll: 0,
            curl_preview_max_scroll: 0,
            response_viewer_scroll: 0,
            response_viewer_max_scroll: 0,
            header_param_names: Vec::new(),
            parameter_param_names: Vec::new(),
        }
    }

    pub fn set_header_param_names(&mut self, names: Vec<String>) {
        self.header_param_names = names;
    }

    pub fn set_parameter_param_names(&mut self, names: Vec<String>) {
        self.parameter_param_names = names;
    }

    pub fn curl_preview_scroll(&self) -> u16 {
        self.curl_preview_scroll
    }

    pub fn set_curl_preview_max_scroll(&mut self, max: u16) {
        self.curl_preview_max_scroll = max;
        if self.curl_preview_scroll > max {
            self.curl_preview_scroll = max;
        }
    }

    pub fn response_viewer_scroll(&self) -> u16 {
        self.response_viewer_scroll
    }

    pub fn set_response_viewer_max_scroll(&mut self, max: u16) {
        self.response_viewer_max_scroll = max;
        if self.response_viewer_scroll > max {
            self.response_viewer_scroll = max;
        }
    }

    pub fn set_operation_count(&mut self, count: usize) {
        self.operation_count = count;
        if count > 0 && self.selected_operation_index >= count {
            self.selected_operation_index = count - 1;
        }
    }

    pub fn set_server_count(&mut self, count: usize) {
        self.server_count = count;
    }

    pub fn sync_selected_operation(&mut self, current_identity: Option<(&str, &str)>) {
        let current = current_identity.map(|(method, path)| (method.to_string(), path.to_string()));
        if current != self.last_selected_identity {
            self.request_builder.reset();
            self.curl_preview_scroll = 0;
            self.last_selected_identity = current;
        }
    }

    fn active_request_editor(&self) -> &PaneEditor {
        match self.request_builder_tab {
            RequestBuilderTab::Header => &self.request_builder.headers,
            RequestBuilderTab::Parameters => &self.request_builder.parameters,
            RequestBuilderTab::Payload => &self.request_builder.payload,
        }
    }

    fn active_request_editor_mut(&mut self) -> &mut PaneEditor {
        match self.request_builder_tab {
            RequestBuilderTab::Header => &mut self.request_builder.headers,
            RequestBuilderTab::Parameters => &mut self.request_builder.parameters,
            RequestBuilderTab::Payload => &mut self.request_builder.payload,
        }
    }

    pub fn set_response(&mut self, response: HttpResponse) {
        self.last_response = Some(response);
        self.response_viewer_scroll = 0;
    }

    pub fn response(&self) -> Option<&HttpResponse> {
        self.last_response.as_ref()
    }

    pub fn is_editing(&self) -> bool {
        match self.focused {
            Pane::RequestBuilder => self.active_request_editor().editing_buffer().is_some(),
            Pane::AuthConfig => self.auth_config.editing_buffer().is_some(),
            Pane::EndpointList => self.filtering,
            Pane::CurlPreview | Pane::ResponseViewer => false,
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Tab | KeyCode::BackTab if !self.is_editing() => self.cycle_focus(key.code),
            KeyCode::Char(digit @ '1'..='5')
                if !self.is_editing()
                    && key.modifiers.contains(KeyModifiers::CONTROL | KeyModifiers::ALT) =>
            {
                self.jump_to_pane(digit)
            }
            KeyCode::Char('[') | KeyCode::Char(']')
                if self.focused == Pane::RequestBuilder && !self.is_editing() =>
            {
                self.cycle_request_builder_tab(key.code == KeyCode::Char(']'))
            }
            _ => match self.focused {
                Pane::EndpointList => self.handle_endpoint_list_key(key.code),
                Pane::RequestBuilder => match self.request_builder_tab {
                    RequestBuilderTab::Header => Self::handle_addable_row_key(
                        &mut self.request_builder.headers,
                        &mut self.request_builder.custom_headers,
                        &self.header_param_names,
                        key.code,
                        key.modifiers,
                    ),
                    RequestBuilderTab::Parameters => Self::handle_addable_row_key(
                        &mut self.request_builder.parameters,
                        &mut self.request_builder.custom_query_params,
                        &self.parameter_param_names,
                        key.code,
                        key.modifiers,
                    ),
                    RequestBuilderTab::Payload => Self::handle_editor_key(
                        &mut self.request_builder.payload,
                        key.code,
                        key.modifiers,
                    ),
                },
                Pane::AuthConfig => {
                    Self::handle_editor_key(&mut self.auth_config, key.code, key.modifiers)
                }
                Pane::CurlPreview => match key.code {
                    KeyCode::Up => {
                        self.curl_preview_scroll = self.curl_preview_scroll.saturating_sub(1)
                    }
                    KeyCode::Down => {
                        self.curl_preview_scroll =
                            (self.curl_preview_scroll + 1).min(self.curl_preview_max_scroll)
                    }
                    _ => {}
                },
                Pane::ResponseViewer => match key.code {
                    KeyCode::Up => {
                        self.response_viewer_scroll = self.response_viewer_scroll.saturating_sub(1)
                    }
                    KeyCode::Down => {
                        self.response_viewer_scroll =
                            (self.response_viewer_scroll + 1).min(self.response_viewer_max_scroll)
                    }
                    _ => {}
                },
            },
        }
    }

    pub fn handle_paste(&mut self, text: &str) {
        match self.focused {
            Pane::RequestBuilder => self.active_request_editor_mut().push_str(text),
            Pane::AuthConfig => self.auth_config.push_str(text),
            Pane::EndpointList | Pane::CurlPreview | Pane::ResponseViewer => {}
        }
    }

    fn handle_endpoint_list_key(&mut self, code: KeyCode) {
        if self.filtering {
            match code {
                KeyCode::Char(c) => self.endpoint_filter.push(c),
                KeyCode::Backspace => {
                    self.endpoint_filter.pop();
                }
                KeyCode::Enter => self.filtering = false,
                KeyCode::Esc => {
                    self.filtering = false;
                    self.endpoint_filter.clear();
                }
                _ => {}
            }
            return;
        }

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
            KeyCode::Char('/') => self.filtering = true,
            _ => {}
        }
        if self.selected_operation_index != before {
            self.request_builder.reset();
        }
    }

    fn handle_editor_key(editor: &mut PaneEditor, code: KeyCode, modifiers: KeyModifiers) {
        match code {
            KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                if editor.editing_buffer().is_some() {
                    editor.commit();
                }
            }
            KeyCode::Up => editor.move_up(),
            KeyCode::Down => editor.move_down(),
            KeyCode::Enter => {
                if editor.editing_buffer().is_some() {
                    if editor.is_editing_multiline_row() {
                        editor.push_char('\n');
                    } else {
                        editor.commit();
                    }
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

    fn handle_addable_row_key(
        editor: &mut PaneEditor,
        custom: &mut Vec<String>,
        existing_spec_names: &[String],
        code: KeyCode,
        modifiers: KeyModifiers,
    ) {
        let add_row_index = editor.row_count().saturating_sub(1);
        let is_commit = (code == KeyCode::Enter && !editor.is_editing_multiline_row())
            || (code == KeyCode::Char('s') && modifiers.contains(KeyModifiers::CONTROL));

        if editor.selected_row() == add_row_index && is_commit && editor.editing_buffer().is_some()
        {
            if let Some(buffer) = editor.editing_buffer() {
                if let Some((name, value)) = buffer.split_once('=') {
                    let name = name.trim().to_string();
                    let value = value.trim().to_string();
                    let is_duplicate =
                        custom.contains(&name) || existing_spec_names.contains(&name);
                    if !name.is_empty() && !is_duplicate {
                        custom.push(name);
                        editor.set_input(add_row_index, value);
                    }
                }
            }
            editor.cancel();
            editor.select_row(add_row_index + 1);
            return;
        }

        Self::handle_editor_key(editor, code, modifiers);
    }

    fn jump_to_pane(&mut self, digit: char) {
        self.focused = match digit {
            '1' => Pane::EndpointList,
            '2' => Pane::AuthConfig,
            '3' => Pane::RequestBuilder,
            '4' => Pane::CurlPreview,
            '5' => Pane::ResponseViewer,
            _ => self.focused,
        };
    }

    fn cycle_request_builder_tab(&mut self, forward: bool) {
        let current_index = REQUEST_BUILDER_TAB_CYCLE
            .iter()
            .position(|tab| *tab == self.request_builder_tab)
            .expect("request_builder_tab is always in REQUEST_BUILDER_TAB_CYCLE");
        let len = REQUEST_BUILDER_TAB_CYCLE.len();

        self.request_builder_tab = if forward {
            REQUEST_BUILDER_TAB_CYCLE[(current_index + 1) % len]
        } else {
            REQUEST_BUILDER_TAB_CYCLE[(current_index + len - 1) % len]
        };
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
