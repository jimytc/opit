use std::collections::{HashMap, HashSet};

pub struct PaneEditor {
    selected_row: usize,
    row_count: usize,
    non_editable_rows: HashSet<usize>,
    multiline_rows: HashSet<usize>,
    inputs: HashMap<usize, String>,
    editing: Option<String>,
}

impl PaneEditor {
    pub fn new() -> Self {
        Self {
            selected_row: 0,
            row_count: 0,
            non_editable_rows: HashSet::new(),
            multiline_rows: HashSet::new(),
            inputs: HashMap::new(),
            editing: None,
        }
    }

    pub fn set_row_count(&mut self, count: usize) {
        self.row_count = count;
        if count > 0 && self.selected_row >= count {
            self.selected_row = count - 1;
        }
    }

    pub fn set_non_editable_rows(&mut self, rows: HashSet<usize>) {
        self.non_editable_rows = rows;
    }

    pub fn set_multiline_rows(&mut self, rows: HashSet<usize>) {
        self.multiline_rows = rows;
    }

    pub fn row_count(&self) -> usize {
        self.row_count
    }

    pub fn set_input(&mut self, row: usize, value: String) {
        self.inputs.insert(row, value);
    }

    pub fn select_row(&mut self, row: usize) {
        self.selected_row = row;
    }

    pub fn is_editing_multiline_row(&self) -> bool {
        self.editing.is_some() && self.multiline_rows.contains(&self.selected_row)
    }

    pub fn selected_row(&self) -> usize {
        self.selected_row
    }

    pub fn editing_buffer(&self) -> Option<&str> {
        self.editing.as_deref()
    }

    pub fn inputs(&self) -> &HashMap<usize, String> {
        &self.inputs
    }

    pub fn inputs_with_live_edit(&self) -> HashMap<usize, String> {
        let mut inputs = self.inputs.clone();
        if let Some(buffer) = &self.editing {
            inputs.insert(self.selected_row, buffer.clone());
        }
        inputs
    }

    pub fn move_up(&mut self) {
        if self.row_count == 0 || self.editing.is_some() {
            return;
        }
        self.selected_row = self.selected_row.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        if self.row_count == 0 || self.editing.is_some() {
            return;
        }
        if self.selected_row < self.row_count - 1 {
            self.selected_row += 1;
        }
    }

    pub fn start_editing(&mut self) {
        if self.row_count == 0 || self.non_editable_rows.contains(&self.selected_row) {
            return;
        }
        self.editing = Some(
            self.inputs
                .get(&self.selected_row)
                .cloned()
                .unwrap_or_default(),
        );
    }

    pub fn push_char(&mut self, c: char) {
        if let Some(buffer) = &mut self.editing {
            buffer.push(c);
        }
    }

    pub fn push_str(&mut self, s: &str) {
        if let Some(buffer) = &mut self.editing {
            buffer.push_str(s);
        }
    }

    pub fn pop_char(&mut self) {
        if let Some(buffer) = &mut self.editing {
            buffer.pop();
        }
    }

    pub fn commit(&mut self) {
        if let Some(buffer) = self.editing.take() {
            self.inputs.insert(self.selected_row, buffer);
        }
    }

    pub fn cancel(&mut self) {
        self.editing = None;
    }

    pub fn reset(&mut self) {
        self.inputs.clear();
        self.editing = None;
        self.selected_row = 0;
    }
}

impl Default for PaneEditor {
    fn default() -> Self {
        Self::new()
    }
}
