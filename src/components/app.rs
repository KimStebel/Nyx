use leptos::prelude::*;
use leptos::web_sys::console;
use wasm_bindgen::JsValue;

use crate::models::Node;
use crate::components::TreeView;

#[component]
pub fn App() -> impl IntoView {
    let child1 = Node::new(false, "bar1", Vec::new());
    let child2 = Node::new(false, "bar2", Vec::new());
    let node = Node::new(false, "foo", vec![child1, child2]);
    
    let log_node_json = move |_| {
        let json = node.to_json();
        // Format JSON with pretty printing (indent of 2 spaces)
        let json_string = serde_json::to_string_pretty(&json).unwrap_or_else(|_| json.to_string());
        console::log_1(&JsValue::from_str(&json_string));
    };

    view! { 
        <div>
            <button on:click=log_node_json>"Log Node JSON"</button>
            <TreeView node />
        </div>
    }
}
