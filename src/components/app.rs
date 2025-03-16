use leptos::prelude::*;

use crate::models::Node;
use crate::components::TreeView;

#[component]
pub fn App() -> impl IntoView {
    let child = Node::new(false, "bar", Vec::new());
    let node = Node::new(false, "foo", vec![child]);

    view! { <TreeView node /> }
}
