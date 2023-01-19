use std::mem;

use sycamore::{builder::prelude::div, prelude::*, utils::render::insert};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};

use crate::{
    hooks::create_id,
    utils::{as_static, class, scoped_children, FocusStartPoint, SetDynAttr},
};

use super::DisclosureProperties;

#[derive(Props)]
pub struct PopoverProps<'cx, G: Html> {
    open: &'cx Signal<bool>,
    #[prop(default)]
    on_open: Option<Box<dyn Fn()>>,
    #[prop(default)]
    on_close: Option<Box<dyn Fn()>>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

pub struct PopoverContext {
    hovering: &'static Signal<bool>,
    owner_id: String,
    button_id: String,
    panel_id: String,
}

#[component]
pub fn Popover<'cx, G: Html>(cx: Scope<'cx>, props: PopoverProps<'cx, G>) -> View<G> {
    let hovering = create_signal(cx, false);
    let owner_id = create_id();
    let button_id = create_id();
    let panel_id = create_id();

    let fsp = FocusStartPoint::new(cx);

    let context = PopoverContext {
        hovering: as_static(hovering),
        owner_id,
        button_id,
        panel_id,
    };
    let disclosure = DisclosureProperties {
        open: as_static(props.open),
        disabled: unsafe { mem::transmute(props.disabled.clone()) },
    };

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(cx, context);
        provide_context(cx, disclosure);
    });
    let class = class(cx, &props.attributes, props.class);

    create_effect(cx, move || {
        if *props.open.get() {
            fsp.save();
            if let Some(on_open) = &props.on_open {
                on_open();
            }
        } else {
            if let Some(on_close) = &props.on_close {
                on_close();
            }
            fsp.load();
        }
    });

    props.attributes.exclude_keys(&["disabled"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_attribute("data-sh".into(), "popover".into());
    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_dyn_bool(cx, "disabled", move || props.disabled.get());
    element.apply_attributes(cx, &props.attributes);
    insert(cx, element, children, None, None, false);

    view
}

#[cfg(test)]
mod tests {
    use sycamore::{builder::prelude::section, prelude::*};

    use super::Popover;

    #[test]
    pub fn test_default_element() {
        let view = sycamore::render_to_string(move |cx| {
            let open = create_signal(cx, false);
            view! { cx, Popover(open = open, attr:data-hello = "hello") { "test" } }
        });

        assert_eq!(
            r#"<div data-hk="1.0" data-sh="popover" class="" data-hello="hello">test</div>"#,
            view
        );
    }

    #[test]
    pub fn test_custom_element() {
        let view = sycamore::render_to_string(move |cx| {
            let open = create_signal(cx, false);
            view! { cx, Popover(open = open, attr:data-hello = "hello", element = section) { "test" } }
        });

        assert_eq!(
            r#"<section data-hk="1.0" data-sh="popover" class="" data-hello="hello">test</section>"#,
            view
        );
    }
}
