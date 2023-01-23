use sycamore::{builder::prelude::label, prelude::*};
use sycamore_utils::{DynamicElement, ReactiveStr};

use crate::utils::{class, SetDynAttr};

use super::ListboxContext;

#[derive(Props)]
pub struct ListboxLabelProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = label.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn ListboxLabel<'cx, G: Html>(cx: Scope<'cx>, props: ListboxLabelProps<'cx, G>) -> View<G> {
    let context: &ListboxContext = use_context(cx);

    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);
    props.attributes.exclude_keys(&["id", "data-sh"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);
    element.set_attribute("data-sh".into(), "listbox-label".into());

    element.set_attribute("id".into(), context.label_id.clone().into());

    view
}
