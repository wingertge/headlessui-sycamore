pub mod focus_navigation;
pub mod focus_navigator;
mod focus_start_point;

use std::{borrow::Cow, mem};

pub use focus_start_point::*;
use sycamore::{
    prelude::*,
    utils::{apply_attribute, render::insert},
    web::html::EventDescriptor,
};
use sycamore_utils::ReactiveStr;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{AddEventListenerOptions, EventTarget};

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
    // SAFETY: This function is used to extend lifetimes to static for use in contexts.
    // Contexts can't outlive the lifetime of the scope so this is safe, they just need to be static
    // for `TypeId` and related functions to work.
    unsafe { mem::transmute(value) }
}

pub trait SetDynAttr<G: Html> {
    fn set_dyn_attr<'cx, N, V, F>(&self, cx: Scope<'cx>, name: N, value: F)
    where
        N: Into<Cow<'static, str>> + 'cx,
        V: Into<Cow<'static, str>> + 'cx,
        F: FnMut() -> V + 'cx;

    fn set_dyn_bool<'cx, N, F>(&self, cx: Scope<'cx>, name: N, value: F)
    where
        N: Into<Cow<'static, str>> + 'cx,
        F: FnMut() -> bool + 'cx;

    fn apply_attributes<'cx>(&self, cx: Scope<'cx>, attrs: &Attributes<'cx, G>);

    fn set_children(&self, cx: Scope<'_>, children: View<G>);
}

impl<G: Html> SetDynAttr<G> for G {
    fn set_dyn_attr<'cx, N, V, F>(&self, cx: Scope<'cx>, name: N, mut value: F)
    where
        N: Into<Cow<'static, str>> + 'cx,
        V: Into<Cow<'static, str>> + 'cx,
        F: FnMut() -> V + 'cx,
    {
        let el = self.clone();
        let name = name.into();
        create_effect(cx, move || el.set_attribute(name.clone(), value().into()));
    }

    fn set_dyn_bool<'cx, N, F>(&self, cx: Scope<'cx>, name: N, mut value: F)
    where
        N: Into<Cow<'static, str>> + 'cx,
        F: FnMut() -> bool + 'cx,
    {
        let el = self.clone();
        let name = name.into();
        create_effect(cx, move || {
            if value() {
                el.set_attribute(name.clone(), "".into());
            } else {
                el.remove_attribute(name.clone());
            }
        });
    }

    fn apply_attributes<'cx>(&self, cx: Scope<'cx>, attrs: &Attributes<'cx, G>) {
        for (name, value) in attrs.drain() {
            apply_attribute(cx, self.clone(), name.clone(), value);
        }
    }

    fn set_children(&self, cx: Scope<'_>, children: View<G>) {
        insert(
            cx,
            self,
            View::new_dyn(cx, move || children.clone()),
            None,
            None,
            false,
        );
    }
}

#[allow(unused)]
pub fn oneshot_event<'a, Ev: EventDescriptor<JsValue>, F: FnMut(Ev::EventData) + 'a>(
    element: &DomNode,
    _ev: Ev,
    mut handler: F,
) {
    let boxed: Box<dyn FnMut(JsValue)> = Box::new(move |ev| handler(ev.into()));
    let boxed: Box<dyn FnMut(JsValue) + 'static> = unsafe { mem::transmute(boxed) };

    let mut options = AddEventListenerOptions::new();
    options.once(true);

    let target: EventTarget = element.to_web_sys().unchecked_into();
    let _ = target.add_event_listener_with_callback_and_add_event_listener_options(
        Ev::EVENT_NAME,
        Closure::new(boxed).as_ref().unchecked_ref(),
        &options,
    );
}
