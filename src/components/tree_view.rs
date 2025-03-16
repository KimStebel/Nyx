use leptos::prelude::*;
use leptos::web_sys::*;

use crate::models::Node;

#[component]
pub fn TreeView(node: Node) -> impl IntoView {
    let is_open = node.is_open.read_only();
    let set_is_open = node.is_open.write_only();
    let text = node.text.read_only();
    let set_text = node.text.write_only();

    let fold_click = move |_ev: MouseEvent| {
        set_is_open.update(|open| *open = !*open);
    };

    let on_blur = move |ev: FocusEvent| {
        if let Some(target) = ev.target() {
            if let Ok(elem) = wasm_bindgen::JsCast::dyn_into::<HtmlElement>(target) {
                let new_text = elem.inner_text().to_string();
                set_text.update(|c| *c = new_text);
            }
        }
    };

    view! {
        <div>
            <span on:blur=on_blur on:click=fold_click style="cursor: pointer">
                "* "
            </span>
            <span contenteditable="true">{text}</span>
            <Show when=move || is_open.get() && !node.children.get().is_empty()>
                <div class="details">
                    <TreeView node=node.children.get().first().unwrap().get() />
                </div>
            </Show>
        </div>
    }
    .into_any()
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_tree_view_initial_state() {
        // Create a simple node tree
        let leaf = Node::new(false, "Leaf Node", vec![]);
        let root = Node::new(true, "Root Node", vec![leaf]);
        
        // Test initial state values
        assert_eq!(root.text.get(), "Root Node");
        assert_eq!(root.is_open.get(), true);
        assert_eq!(root.children.get().len(), 1);
        
        let child = root.children.get().first().unwrap().get();
        assert_eq!(child.text.get(), "Leaf Node");
        assert_eq!(child.is_open.get(), false);
    }

    #[wasm_bindgen_test]
    fn test_tree_view_toggle() {
        // Create node with initial open state
        let node = Node::new(true, "Test Node", vec![]);
        
        // Get writable signal
        let set_is_open = node.is_open.write_only();
        
        // Toggle closed
        set_is_open.update(|open| *open = !*open);
        assert_eq!(node.is_open.get(), false);
        
        // Toggle open again
        set_is_open.update(|open| *open = !*open);
        assert_eq!(node.is_open.get(), true);
    }

    #[wasm_bindgen_test]
    fn test_tree_view_text_update() {
        // Create a node for testing
        let node = Node::new(false, "Initial Text", vec![]);
        
        // Get writable signal
        let set_text = node.text.write_only();
        
        // Update text
        let new_text = "Updated Text";
        set_text.update(|c| *c = new_text.to_string());
        
        // Verify update
        assert_eq!(node.text.get(), new_text);
    }

    #[wasm_bindgen_test]
    fn test_tree_view_children_access() {
        // Create a node with children
        let child1 = Node::new(false, "Child 1", vec![]);
        let child2 = Node::new(false, "Child 2", vec![]);
        let parent = Node::new(false, "Parent Node", vec![child1, child2]);
        
        // Test children count
        assert_eq!(parent.children.get().len(), 2);
        
        // Test children access
        let first_child = parent.children.get().first().unwrap().get();
        assert_eq!(first_child.text.get(), "Child 1");
        
        let second_child = parent.children.get().get(1).unwrap().get();
        assert_eq!(second_child.text.get(), "Child 2");
    }

    #[wasm_bindgen_test]
    fn test_tree_view_nested_children() {
        // Create a deeply nested tree structure
        let grandchild = Node::new(false, "Grandchild", vec![]);
        let child = Node::new(true, "Child", vec![grandchild]);
        let parent = Node::new(true, "Parent", vec![child]);
        
        // Access nested children
        let child_node = parent.children.get().first().unwrap().get();
        assert_eq!(child_node.text.get(), "Child");
        
        let grandchild_node = child_node.children.get().first().unwrap().get();
        assert_eq!(grandchild_node.text.get(), "Grandchild");
    }

    #[wasm_bindgen_test]
    fn test_tree_view_empty_children() {
        // Test node with no children
        let node = Node::new(true, "Empty Node", vec![]);
        
        // Verify children count
        assert_eq!(node.children.get().len(), 0);
        
        // Verify children collection is empty
        assert!(node.children.get().is_empty());
    }

    #[wasm_bindgen_test]
    fn test_tree_view_add_child() {
        // Create a parent node with no children initially
        let parent = Node::new(true, "Parent", vec![]);
        
        // Create a child node
        let child = Node::new(false, "New Child", vec![]);
        
        // Add child to parent
        let mut children = parent.children.get();
        children.push(RwSignal::new(child));
        parent.children.set(children);
        
        // Verify child was added
        assert_eq!(parent.children.get().len(), 1);
        
        // Verify child properties
        let added_child = parent.children.get().first().unwrap().get();
        assert_eq!(added_child.text.get(), "New Child");
        assert_eq!(added_child.is_open.get(), false);
    }
}
