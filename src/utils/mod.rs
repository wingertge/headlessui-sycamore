pub mod focus_navigation;
pub mod focus_navigator;
mod focus_start_point;

use std::mem;

pub use focus_start_point::*;
use sycamore::prelude::*;
use sycamore_utils::ReactiveStr;

pub fn scoped_children<'a, G: Html, F>(cx: Scope<'a>, children: Children<'a, G>, f: F) -> View<G>
where
    for<'b> F: FnOnce(Scope<'b>),
{
    let mut view = View::empty();
    create_child_scope(cx, |cx| {
        f(cx);
        view = children.call(cx);
    });
    view
}

pub fn class<'cx, G: Html>(
    cx: Scope<'cx>,
    attributes: &Attributes<'cx, G>,
    prop: ReactiveStr<'cx>,
) -> &'cx ReadSignal<String> {
    let mut attr_class = attributes.remove("class");
    create_memo(cx, move || {
        let attr_class = attr_class.as_mut().map(|class| match class {
            AttributeValue::Str(s) => s.to_string(),
            AttributeValue::DynamicStr(s) => s(),
            _ => unreachable!(),
        });
        attr_class.unwrap_or_else(|| prop.get())
    })
}

pub fn get_ref<'cx, G: Html>(cx: Scope<'cx>, attributes: &Attributes<'cx, G>) -> &'cx NodeRef<G> {
    attributes
        .remove_ref()
        .unwrap_or_else(|| create_node_ref(cx))
}

pub fn as_static<T>(value: &T) -> &'static T {
    unsafe { mem::transmute(value) }
}
