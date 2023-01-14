use sycamore::prelude::*;

use crate::{components::BaseProps, utils::class};

use super::ListboxContext;

#[component]
pub fn ListboxLabel<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    let context: &ListboxContext = use_context(cx);

    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);
    props.attributes.exclude_keys(&["id", "data-sh"]);

    view! { cx,
        label(id = context.label_id, data-sh = "listbox-label", class = class, ..props.attributes) {
            (children)
        }
    }
}
