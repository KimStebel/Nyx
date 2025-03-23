use leptos::prelude::*;

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
}
