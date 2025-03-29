use leptos::prelude::*;
use serde_json::{json, Value};

#[derive(Clone, Copy)]
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

    pub fn from_json(value: &Value) -> Option<Self> {
        let id = value["id"].as_u64()?.try_into().ok()?;
        let is_open = value["is_open"].as_bool()?;
        let text = value["text"].as_str()?.to_string();

        let children_json = value["children"].as_array()?;
        let children: Vec<Node> = children_json
            .iter()
            .filter_map(|child_json| Node::from_json(child_json))
            .collect();

        Some(Self {
            id: RwSignal::new(id),
            is_open: RwSignal::new(is_open),
            text: RwSignal::new(text),
            children: RwSignal::new(children.into_iter().map(RwSignal::new).collect()),
        })
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
        let children: Vec<Value> = self
            .children
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

    pub fn save_to_local_storage(&self, key: &str) -> Result<(), String> {
        let json_value = self.to_json();
        let json_string = json_value.to_string();

        let window = window();

        let storage = window
            .local_storage()
            .map_err(|_| "Failed to access localStorage".to_string())?
            .ok_or_else(|| "localStorage not available".to_string())?;

        storage
            .set_item(key, &json_string)
            .map_err(|err| format!("Failed to set localStorage item: {:?}", err))?;

        Ok(())
    }

    pub fn load_from_local_storage(key: &str) -> Result<Self, String> {
        let window = window();

        let storage = window
            .local_storage()
            .map_err(|_| "Failed to access localStorage".to_string())?
            .ok_or_else(|| "localStorage not available".to_string())?;

        let json_string = storage
            .get_item(key)
            .map_err(|_| "Failed to get item from localStorage".to_string())?
            .ok_or_else(|| format!("No item found with key: {}", key))?;

        let json_value: Value = serde_json::from_str(&json_string)
            .map_err(|err| format!("Failed to parse JSON: {}", err))?;

        Self::from_json(&json_value).ok_or_else(|| "Failed to convert JSON to Node".to_string())
    }

    pub fn remove_from_local_storage(key: &str) -> Result<(), String> {
        let window = window();

        let storage = window
            .local_storage()
            .map_err(|_| "Failed to access localStorage".to_string())?
            .ok_or_else(|| "localStorage not available".to_string())?;

        storage
            .remove_item(key)
            .map_err(|err| format!("Failed to remove localStorage item: {:?}", err))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

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

    #[test]
    fn test_from_json() {
        // Create a JSON representation
        let json = json!({
            "id": 123,
            "is_open": true,
            "text": "Parent",
            "children": [
                {
                    "id": 456,
                    "is_open": false,
                    "text": "Child",
                    "children": []
                }
            ]
        });

        // Parse JSON to Node
        let node = Node::from_json(&json).unwrap();

        // Verify top-level properties
        assert_eq!(node.id.get(), 123);
        assert_eq!(node.is_open.get(), true);
        assert_eq!(node.text.get(), "Parent");

        // Verify children
        let children = node.children.get();
        assert_eq!(children.len(), 1);

        // Verify child properties
        let child = children[0].get();
        assert_eq!(child.id.get(), 456);
        assert_eq!(child.is_open.get(), false);
        assert_eq!(child.text.get(), "Child");
        assert_eq!(child.children.get().len(), 0);
    }

    #[test]
    fn test_json_roundtrip() {
        // Create a nested node structure
        let grandchild = Node::new(false, "Grandchild", vec![]);
        let child = Node::new(true, "Child", vec![grandchild]);
        let original = Node::new(true, "Parent", vec![child]);

        // Convert to JSON and back
        let json = original.to_json();
        let roundtrip = Node::from_json(&json).unwrap();

        // Verify properties survived the roundtrip
        assert_eq!(roundtrip.id.get(), original.id.get());
        assert_eq!(roundtrip.is_open.get(), original.is_open.get());
        assert_eq!(roundtrip.text.get(), original.text.get());

        // Verify children
        let original_children = original.children.get();
        let roundtrip_children = roundtrip.children.get();
        assert_eq!(roundtrip_children.len(), original_children.len());

        // Verify first child
        let original_child = original_children[0].get();
        let roundtrip_child = roundtrip_children[0].get();
        assert_eq!(roundtrip_child.id.get(), original_child.id.get());
        assert_eq!(roundtrip_child.is_open.get(), original_child.is_open.get());
        assert_eq!(roundtrip_child.text.get(), original_child.text.get());
    }

    #[wasm_bindgen_test]
    fn test_local_storage_save_and_load() {
        // Create a unique key for this test to avoid conflicts
        let test_key = format!("test_node_{}", js_sys::Date::now());

        // Create a node to save
        let child = Node::new(false, "Test Child", vec![]);
        let original = Node::new(true, "Test Parent", vec![child]);

        // Save to localStorage
        let save_result = original.save_to_local_storage(&test_key);
        assert!(save_result.is_ok(), "Saving to localStorage should succeed");

        // Load from localStorage
        let loaded_result = Node::load_from_local_storage(&test_key);
        assert!(
            loaded_result.is_ok(),
            "Loading from localStorage should succeed"
        );

        let loaded = loaded_result.unwrap();

        // Verify properties preserved
        assert_eq!(loaded.id.get(), original.id.get());
        assert_eq!(loaded.is_open.get(), original.is_open.get());
        assert_eq!(loaded.text.get(), original.text.get());

        // Verify children
        let original_children = original.children.get();
        let loaded_children = loaded.children.get();
        assert_eq!(loaded_children.len(), original_children.len());

        // Verify child properties
        let original_child = original_children[0].get();
        let loaded_child = loaded_children[0].get();
        assert_eq!(loaded_child.id.get(), original_child.id.get());
        assert_eq!(loaded_child.is_open.get(), original_child.is_open.get());
        assert_eq!(loaded_child.text.get(), original_child.text.get());

        // Clean up
        let remove_result = Node::remove_from_local_storage(&test_key);
        assert!(
            remove_result.is_ok(),
            "Removing from localStorage should succeed"
        );

        // Verify removal
        let load_after_remove = Node::load_from_local_storage(&test_key);
        assert!(
            load_after_remove.is_err(),
            "Loading after removal should fail"
        );
    }
}
