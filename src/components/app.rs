use leptos::prelude::*;

use crate::models::Node;
use crate::components::TreeView;

#[component]
pub fn App() -> impl IntoView {
    let child1 = Node::new(false, "bar1", Vec::new());
    let child2 = Node::new(false, "bar2", Vec::new());
    let node = Node::new(false, "foo", vec![child1, child2]);

    view! { <TreeView node /> }
}
