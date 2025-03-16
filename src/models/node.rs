use leptos::prelude::*;

#[derive(Clone)]
pub struct Node {
    pub is_open: RwSignal<bool>,
    pub text: RwSignal<String>,
    pub children: RwSignal<Vec<RwSignal<Node>>>,
}

impl Node {
    pub fn new(is_open: bool, text: &str, children: Vec<Node>) -> Self {
        let child_signals: Vec<RwSignal<Node>> = children
            .into_iter()
            .map(|child_node| RwSignal::new(child_node))
            .collect();

        Self {
            is_open: RwSignal::new(is_open),
            text: RwSignal::new(text.to_string()),
            children: RwSignal::new(child_signals),
        }
    }
}
