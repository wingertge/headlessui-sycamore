use std::mem;

use sycamore::{
    builder::prelude::{button, div},
    prelude::*,
    rt::JsCast,
    web::html::ev,
};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
use web_sys::{FocusEvent, KeyboardEvent};

use crate::{
    hooks::create_id,
    utils::{
        as_static, class,
        focus_navigation::{as_html_element, focus_first, get_focusable_elements, lock_focus},
        get_ref, scoped_children, FocusStartPoint, SetDynAttr,
    },
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
    pub owner_id: String,
    pub button_id: String,
    pub panel_id: String,
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
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

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

#[derive(Props)]
pub struct PopoverButtonProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default = button.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn PopoverButton<'cx, G: Html>(cx: Scope<'cx>, props: PopoverButtonProps<'cx, G>) -> View<G> {
    let context: &PopoverContext = use_context(cx);
    let properties: &DisclosureProperties = use_context(cx);

    let children = props.children.call(cx);
    let class = class(cx, &props.attributes, props.class);

    props.attributes.exclude_keys(&[
        "on:click",
        "on:mouseenter",
        "on:mouseleave",
        "id",
        "disabled",
        "expanded",
        "aria-controls",
    ]);

    let view = props.element.call(cx);
    let element = create_ref(cx, view.as_node().unwrap().clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.event(cx, ev::click, {
        let disabled = props.disabled.clone();
        move |_| {
            if !properties.disabled.get() && !disabled.get() {
                properties.open.set(!*properties.open.get());
            }
        }
    });
    element.event(cx, ev::mouseenter, move |_| context.hovering.set(true));
    element.event(cx, ev::mouseleave, move |_| context.hovering.set(false));

    element.set_attribute("id".into(), context.button_id.clone().into());
    element.set_attribute("data-sh".into(), "popover-button".into());
    element.set_dyn_bool(cx, "disabled", move || {
        properties.disabled.get() || props.disabled.get()
    });
    element.set_dyn_bool(cx, "expanded", move || *properties.open.get());
    create_effect(cx, move || {
        if *properties.open.get() {
            element.set_attribute("aria-controls".into(), context.panel_id.clone().into());
        } else {
            element.remove_attribute("aria-controls".into());
        }
    });

    view
}

#[derive(Props)]
pub struct PopoverOverlayProps<'cx, G: Html> {
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn PopoverOverlay<'cx, G: Html>(cx: Scope<'cx>, props: PopoverOverlayProps<'cx, G>) -> View<G> {
    let properties: &DisclosureProperties = use_context(cx);

    let children = props.children.call(cx);
    let class = class(cx, &props.attributes, props.class);

    props.attributes.exclude_keys(&["on:click"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh".into(), "popover-overlay".into());

    element.event(cx, ev::click, move |_| properties.open.set(false));

    view
}

#[derive(Props)]
pub struct PopoverPanelProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn PopoverPanel<'cx, G: Html>(cx: Scope<'cx>, props: PopoverPanelProps<'cx, G>) -> View<G> {
    let context: &PopoverContext = use_context(cx);
    let properties: &DisclosureProperties = use_context(cx);

    let node = get_ref(cx, &props.attributes);

    create_effect(cx, move || {
        if *properties.open.get() {
            if let Some(elements) = get_focusable_elements(node) {
                focus_first(elements);
            }
        }
    });

    let children = props.children.call(cx);
    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    node.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("id".into(), context.panel_id.clone().into());
    element.set_attribute("data-sh".into(), "popover-panel".into());

    element.event(cx, ev::keydown, {
        let disabled = props.disabled.clone();
        move |e: KeyboardEvent| {
            if !disabled.get() {
                match e.key().as_str() {
                    "Tab" => {
                        e.prevent_default();
                        lock_focus(node, e.shift_key());
                    }
                    "Escape" => {
                        properties.open.set(false);
                    }
                    _ => {}
                }
            }
        }
    });
    element.event(cx, ev::focusout, move |e: FocusEvent| {
        if !*context.hovering.get() {
            match (as_html_element(node), e.related_target()) {
                (_, None) => properties.open.set(false),
                (Some(node), related)
                    if !node.contains(related.as_ref().and_then(|related| related.dyn_ref())) =>
                {
                    properties.open.set(false)
                }
                _ => {}
            };
        }
    });

    View::new_dyn(cx, move || {
        if *create_selector(cx, move || *properties.open.get()).get() {
            view.clone()
        } else {
            View::empty()
        }
    })
}
