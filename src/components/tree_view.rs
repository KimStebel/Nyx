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
