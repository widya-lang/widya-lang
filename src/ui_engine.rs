//! Declarative Reactive UI Engine (ui_engine.rs)
//! Mendukung komponen UI deklaratif (Column, Row, Text, Button, Container), State Management reaktif, dan diffing layout tree.

use std::collections::HashMap;

/// Elemen Virtual Node Pohon UI
#[derive(Debug, Clone, PartialEq)]
pub enum UiNode {
    Text {
        content: String,
        font_size: f32,
        color_hex: String,
    },
    Button {
        label: String,
        on_click_id: String,
    },
    Input {
        placeholder: String,
        value: String,
        on_change_id: String,
    },
    Column {
        children: Vec<UiNode>,
        spacing: f32,
    },
    Row {
        children: Vec<UiNode>,
        spacing: f32,
    },
    Container {
        child: Box<UiNode>,
        padding: f32,
        background_hex: String,
    },
}

/// Status & Reaktifitas Komponen UI
#[derive(Debug, Clone)]
pub struct UiStateStore {
    state_map: HashMap<String, String>,
    dirty: bool,
}

impl UiStateStore {
    pub fn new() -> Self {
        Self {
            state_map: HashMap::new(),
            dirty: false,
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        let changed = self.state_map.get(key).map(|v| v != value).unwrap_or(true);
        if changed {
            self.state_map.insert(key.to_string(), value.to_string());
            self.dirty = true;
        }
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.state_map.get(key)
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}

/// Renderer Komponen UI Deklaratif
pub struct ReactiveUiRenderer;

impl ReactiveUiRenderer {
    /// Menghitung jumlah node total di dalam virtual tree
    pub fn count_nodes(root: &UiNode) -> usize {
        match root {
            UiNode::Text { .. } | UiNode::Button { .. } | UiNode::Input { .. } => 1,
            UiNode::Column { children, .. } | UiNode::Row { children, .. } => {
                1 + children.iter().map(Self::count_nodes).sum::<usize>()
            }
            UiNode::Container { child, .. } => 1 + Self::count_nodes(child),
        }
    }

    /// Dispatch event klik tombol
    pub fn handle_event(root: &UiNode, event_action_id: &str, state: &mut UiStateStore) -> bool {
        match root {
            UiNode::Button { on_click_id, .. } => {
                if on_click_id == event_action_id {
                    state.set("last_clicked", event_action_id);
                    return true;
                }
                false
            }
            UiNode::Column { children, .. } | UiNode::Row { children, .. } => {
                for child in children {
                    if Self::handle_event(child, event_action_id, state) {
                        return true;
                    }
                }
                false
            }
            UiNode::Container { child, .. } => Self::handle_event(child, event_action_id, state),
            _ => false,
        }
    }
}
