use leptos::logging::log;
use leptos::prelude::*;
use leptos::web_sys::console;
use wasm_bindgen::JsValue;

use crate::components::TreeView;
use crate::models::Node;

fn create_default_node() -> Node {
    let child1 = Node::new(false, "bar1", Vec::new());
    let child2 = Node::new(false, "bar2", Vec::new());
    Node::new(false, "foo", vec![child1, child2])
}

#[component]
pub fn App() -> impl IntoView {
    log!("starting...");
    
    // Try to load from localStorage, fall back to default if it fails
    let node = match Node::load_from_local_storage("root") {
        Ok(loaded_node) => {
            log!("Loaded node from localStorage");
            loaded_node
        }
        Err(err) => {
            log!("Failed to load from localStorage: {}, using default", err);
            create_default_node()
        }
    };
    
    let log_node_json = move |_| {
        let json = node.to_json();
        // Format JSON with pretty printing (indent of 2 spaces)
        let json_string = serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string());
        console::log_1(&JsValue::from_str(&json_string));
    };

    let save_to_storage = move |_| match node.save_to_local_storage("root") {
        Ok(_) => console::log_1(&JsValue::from_str(
            "Node saved to localStorage with key 'root'",
        )),
        Err(err) => console::error_1(&JsValue::from_str(&format!("Error saving node: {}", err))),
    };

    view! {
        <div>
            <button on:click=log_node_json>"Log Node JSON"</button>
            <button on:click=save_to_storage>"Save to localStorage"</button>
            <TreeView node />
        </div>
    }
}
