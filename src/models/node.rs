use leptos::prelude::*;
use serde_json::{json, Value};

#[derive(Clone)]
#[derive(Copy)]
pub struct Node {
    pub id: RwSignal<usize>,
    pub is_open: RwSignal<bool>,
    pub text: RwSignal<String>,
    pub children: RwSignal<Vec<RwSignal<Node>>>,
}

impl Node {
    pub fn new(is_open: bool, text: &str, children: Vec<Node>) -> Self {
        static NEXT_ID: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);
        let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        let child_signals: Vec<RwSignal<Node>> = children
            .into_iter()
            .map(|child_node| RwSignal::new(child_node))
            .collect();

        Self {
            id: RwSignal::new(id),
            is_open: RwSignal::new(is_open),
            text: RwSignal::new(text.to_string()),
            children: RwSignal::new(child_signals),
        }
    }
    
    pub fn id(&self) -> usize {
        self.id.get()
    }

    pub fn prepend_child(&self, child: Node) {
        let child_signal = RwSignal::new(child);
        self.children.update(|children| {
            children.insert(0, child_signal);
        });
    }

    pub fn remove_child(&self, id: usize) -> bool {
        let mut success = false;
        self.children.update(|children| {
            if let Some(index) = children.iter().position(|child| child.get().id() == id) {
                children.remove(index);
                success = true;
            }
        });
        success
    }

    pub fn to_json(&self) -> Value {
        let children: Vec<Value> = self.children
            .get()
            .iter()
            .map(|child_signal| child_signal.get().to_json())
            .collect();

        json!({
            "id": self.id.get(),
            "is_open": self.is_open.get(),
            "text": self.text.get(),
            "children": children
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prepend_child() {
        let child1 = Node::new(false, "Child 1", vec![]);
        let child2 = Node::new(false, "Child 2", vec![]);
        let node = Node::new(true, "Parent", vec![child1]);
        
        // Check initial state
        let children = node.children.get();
        assert_eq!(children.len(), 1);
        assert_eq!(children[0].get().text.get(), "Child 1");
        
        // Prepend child
        node.prepend_child(child2);
        
        // Check updated state
        let updated_children = node.children.get();
        assert_eq!(updated_children.len(), 2);
        assert_eq!(updated_children[0].get().text.get(), "Child 2");
        assert_eq!(updated_children[1].get().text.get(), "Child 1");
    }

    #[test]
    fn test_remove_child() {
        // Create parent with two children
        let child1 = Node::new(false, "Child 1", vec![]);
        let child2 = Node::new(false, "Child 2", vec![]);
        let node = Node::new(true, "Parent", vec![child1, child2]);
        
        // Get child IDs
        let children = node.children.get();
        let child1_id = children[0].get().id();
        let child2_id = children[1].get().id();
        
        // Remove first child
        let result = node.remove_child(child1_id);
        assert!(result);
        
        // Check child was removed
        let updated_children = node.children.get();
        assert_eq!(updated_children.len(), 1);
        assert_eq!(updated_children[0].get().id(), child2_id);
        
        // Try to remove non-existent child
        let result = node.remove_child(9999);
        assert!(!result);
        
        // Check children count remains unchanged
        assert_eq!(node.children.get().len(), 1);
    }

    #[test]
    fn test_to_json() {
        // Create a nested node structure
        let grandchild = Node::new(false, "Grandchild", vec![]);
        let child = Node::new(true, "Child", vec![grandchild]);
        let parent = Node::new(true, "Parent", vec![child]);
        
        // Get JSON representation
        let json = parent.to_json();
        
        // Verify top-level properties
        assert_eq!(json["text"], "Parent");
        assert_eq!(json["is_open"], true);
        assert!(json["id"].is_number());
        
        // Verify child structure
        let children = &json["children"];
        assert!(children.is_array());
        assert_eq!(children.as_array().unwrap().len(), 1);
        
        // Verify first child properties
        let first_child = &children[0];
        assert_eq!(first_child["text"], "Child");
        assert_eq!(first_child["is_open"], true);
        
        // Verify grandchild structure
        let grandchildren = &first_child["children"];
        assert!(grandchildren.is_array());
        assert_eq!(grandchildren.as_array().unwrap().len(), 1);
        
        // Verify grandchild properties
        let first_grandchild = &grandchildren[0];
        assert_eq!(first_grandchild["text"], "Grandchild");
        assert_eq!(first_grandchild["is_open"], false);
        assert!(first_grandchild["children"].is_array());
        assert!(first_grandchild["children"].as_array().unwrap().is_empty());
    }
}
