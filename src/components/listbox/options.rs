use std::{cell::RefCell, hash::Hash};

use super::ListboxContext;
use crate::{
    components::{
        select::SelectProperties, DisclosureProperties, TransitionContext, TransitionProp,
    },
    utils::{as_static, class, get_ref, SetDynAttr},
    FocusNavigator,
};
use gloo_timers::callback::Timeout;
use sycamore::{
    builder::prelude::{li, ul},
    prelude::*,
    rt::JsCast,
    web::html::ev,
};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
use web_sys::{FocusEvent, HtmlElement, KeyboardEvent};

#[derive(Props)]
pub struct ListboxOptionsProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default)]
    transition: Option<TransitionProp<'cx, G>>,
    #[prop(default = ul.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn ListboxOptions<'cx, T: Clone + Hash + Eq + 'static, G: Html>(
    cx: Scope<'cx>,
    props: ListboxOptionsProps<'cx, G>,
) -> View<G> {
    let context: &ListboxContext = use_context(cx);
    let select_properties: &SelectProperties<T> = use_context(cx);
    let properties: &DisclosureProperties = use_context(cx);

    let internal_ref = get_ref(cx, &props.attributes);
    let controller = FocusNavigator::new(context.owner_id.clone(), internal_ref);

    create_effect(cx, move || {
        if !select_properties.has_selected() {
            controller.set_first_checked();
        }
    });

    let on_blur = move |e: FocusEvent| {
        let element = internal_ref
            .get::<DomNode>()
            .unchecked_into::<HtmlElement>();
        if !*context.hovering.get()
            && (e.related_target().is_none()
                || !element.contains(e.related_target().as_ref().map(|el| el.dyn_ref().unwrap())))
        {
            properties.open.set(false);
        }
    };

    let disabled = create_memo(cx, move || {
        properties.disabled.get() || props.disabled.get()
    });
    let tabindex = create_memo(cx, move || if *disabled.get() { -1 } else { 0 });
    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);

    props.attributes.exclude_keys(&[
        "on:focusout",
        "data-sh",
        "id",
        "role",
        "aria-multiselectable",
        "aria-labelledby",
        "aria-orientation",
        "tabindex",
        "disabled",
    ]);

    let apply_attributes = |element: &G| {
        internal_ref.set(element.clone());

        element.set_dyn_attr(cx, "class", move || class.to_string());
        element.set_children(cx, children);
        element.apply_attributes(cx, &props.attributes);
        element.set_attribute("data-sh".into(), "listbox-options".into());

        element.set_attribute("id".into(), context.options_id.clone().into());
        element.set_attribute("role".into(), "listbox".into());
        element.set_attribute("aria-labelledby".into(), context.button_id.clone().into());
        element.set_dyn_attr(cx, "tabindex", move || tabindex.to_string());
        element.set_dyn_bool(cx, "aria-multiselectable", move || context.multiple);
        element.set_dyn_bool(cx, "disabled", move || *disabled.get());
        element.set_attribute(
            "aria-orientation".into(),
            if context.horizontal {
                "horizontal"
            } else {
                "vertical"
            }
            .into(),
        );

        element.event(cx, ev::focusout, on_blur);
    };

    if let Some(transition) = props.transition {
        let mut view = View::empty();
        let node_ref = create_node_ref(cx);
        create_child_scope(cx, |cx| {
            provide_context(
                cx,
                TransitionContext::<G> {
                    node_ref: as_static(node_ref),
                },
            );
            view = transition(cx, as_static(properties.open));
        });
        let element = node_ref.get_raw();
        apply_attributes(&element);
        view
    } else {
        let view = props.element.call(cx);
        let element = view.as_node().unwrap();
        apply_attributes(element);

        view! { cx,
            (if *properties.open.get() {
                view.clone()
            } else {
                View::empty()
            })
        }
    }
}

#[derive(Props)]
pub struct ListboxOptionProps<'cx, T: Eq + Hash + 'static, G: Html> {
    value: T,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = li.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn ListboxOption<'cx, T: Clone + Eq + Hash + 'static, G: Html>(
    cx: Scope<'cx>,
    props: ListboxOptionProps<'cx, T, G>,
) -> View<G> {
    let context: &ListboxContext = use_context(cx);
    let focus: &FocusNavigator<G> = as_static(use_context(cx));
    let disclosure: &DisclosureProperties = use_context(cx);
    let properties: &SelectProperties<T> = as_static(use_context(cx));

    let value = as_static(create_ref(cx, props.value));

    let characters = as_static(create_ref(cx, RefCell::new(String::new())));
    let delay = as_static(create_ref::<RefCell<Option<Timeout>>>(
        cx,
        RefCell::new(None),
    ));

    on_cleanup(cx, || {
        if let Some(delay) = delay.borrow_mut().take() {
            delay.cancel();
        }
    });

    let disabled = create_memo(cx, move || {
        properties.disabled.get() || props.disabled.get()
    });
    let node = get_ref(cx, &props.attributes);

    let on_key_down = move |e: KeyboardEvent| {
        if !*disabled.get() {
            match e.key().as_str() {
                "ArrowLeft" if context.horizontal => {
                    e.prevent_default();
                    focus.set_prev_checked(node, false);
                }
                "ArrowUp" if !context.horizontal => {
                    e.prevent_default();
                    focus.set_prev_checked(node, false);
                }
                "ArrowRight" if context.horizontal => {
                    e.prevent_default();
                    focus.set_next_checked(node, false);
                }
                "ArrowDown" if !context.horizontal => {
                    e.prevent_default();
                    focus.set_next_checked(node, false);
                }
                " " | "Enter" => {
                    properties.select(value.clone());
                    if !context.multiple {
                        e.prevent_default();
                        disclosure.open.set(false);
                    }
                }
                "Home" => {
                    e.prevent_default();
                    focus.set_first_checked();
                }
                "End" => {
                    e.prevent_default();
                    focus.set_last_checked();
                }
                key if key.len() == 1 => {
                    characters.borrow_mut().push_str(key);
                    if let Some(timeout) = delay.borrow_mut().take() {
                        timeout.cancel();
                    }
                    *delay.borrow_mut() = Some(Timeout::new(100, move || {
                        focus.set_first_match(characters.borrow().as_ref());
                        characters.borrow_mut().clear();
                    }));
                }
                _ => {}
            }
        }
    };

    let on_click = move |_| {
        if !*disabled.get() {
            properties.select(value.clone());
            if !context.multiple {
                disclosure.open.set(false);
            }
        }
    };

    let on_focus = move |_| {
        if !*disabled.get() {
            properties.focus(value.clone())
        }
    };

    let on_blur = move |_| {
        if !*disabled.get() {
            properties.blur();
        }
    };

    create_effect(cx, move || {
        if let Some(element) = node
            .try_get::<DomNode>()
            .map(|node| node.to_web_sys())
            .as_ref()
            .and_then(|node| node.dyn_ref::<HtmlElement>())
        {
            if *disclosure.open.get() && properties.is_selected_untracked(value) && !*disabled.get()
            {
                let _ = element.focus();
            }
        }
    });

    let selected = create_ref(cx, move || properties.is_selected(value));
    let class = class(cx, &props.attributes, props.class);
    let children = props.children.call(cx);

    props.attributes.exclude_keys(&[
        "on:keydown",
        "on:click",
        "on:focus",
        "on:blur",
        "role",
        "tabindex",
        "ref",
        "disabled",
        "aria-selected",
    ]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    node.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);
    element.set_attribute("data-sh".into(), "listbox-option".into());

    element.set_attribute("role".into(), "option".into());
    element.set_attribute("tabindex".into(), "-1".into());
    element.set_attribute("data-sh-owner".into(), context.owner_id.clone().into());
    element.set_dyn_bool(cx, "disabled", move || *disabled.get());
    element.set_dyn_bool(cx, "aria-selected", selected);
    element.set_dyn_bool(cx, "data-sh-selected", selected);
    element.set_dyn_bool(cx, "data-sh-active", move || properties.is_active(value));

    element.event(cx, ev::keydown, on_key_down);
    element.event(cx, ev::click, on_click);
    element.event(cx, ev::focus, on_focus);
    element.event(cx, ev::blur, on_blur);

    view
}
