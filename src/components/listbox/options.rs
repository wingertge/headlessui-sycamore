use std::{cell::RefCell, hash::Hash, rc::Rc, time::Duration};

use super::ListboxContext;
use crate::{
    components::{select::HeadlessSelectProperties, DisclosureProperties},
    utils::{as_static, class, get_ref},
    FocusNavigator,
};
use fluvio_wasm_timer::Delay;
use sycamore::{prelude::*, rt::JsCast};
use sycamore_utils::{ReactiveBool, ReactiveStr};
use web_sys::{FocusEvent, HtmlElement, KeyboardEvent};

#[derive(Props)]
pub struct ListboxOptionsProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn ListboxOptions<'cx, T: Clone + Hash + Eq + 'static, G: Html>(
    cx: Scope<'cx>,
    props: ListboxOptionsProps<'cx, G>,
) -> View<G> {
    let context: &ListboxContext = use_context(cx);
    let select_properties: &HeadlessSelectProperties<T> = use_context(cx);
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

    view! { cx,
        ul(
            on:focusout = on_blur, data-sh = "listbox-options", id = context.options_id, role = "listbox",
            aria-multiselectable = context.multiple, aria-labelledby = context.button_id, ref = internal_ref,
            aria-orientation = if context.horizontal { "horizontal" } else { "vertical" },
            tabindex = tabindex, disabled = *disabled.get(), class = class, ..props.attributes
        ) {
            (children)
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
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn ListboxOption<'cx, T: Eq + Hash + 'static, G: Html>(
    cx: Scope<'cx>,
    props: ListboxOptionProps<'cx, T, G>,
) -> View<G> {
    let context: &ListboxContext = use_context(cx);
    let focus: &FocusNavigator<G> = as_static(use_context(cx));
    let disclosure: &DisclosureProperties = use_context(cx);
    let properties: &HeadlessSelectProperties<T> = as_static(use_context(cx));

    let value = as_static(create_ref(cx, Rc::new(props.value)));

    let characters = as_static(create_ref(cx, RefCell::new(String::new())));
    let delay = as_static(create_ref::<RefCell<Option<Delay>>>(cx, RefCell::new(None)));

    on_cleanup(cx, || {
        *delay.borrow_mut() = None;
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
                    if let Some(timeout) = delay.borrow_mut().as_mut() {
                        timeout.reset(Duration::from_millis(100));
                    } else {
                        *delay.borrow_mut() = Some(Delay::new(Duration::from_millis(100)));
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Some(delay) = delay.borrow_mut().as_mut() {
                                if let Ok(_) = delay.await {
                                    focus.set_first_match(characters.borrow().as_ref());
                                    characters.borrow_mut().clear();
                                }
                            }
                        });
                    }
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
            if *disclosure.open.get()
                && properties.is_selected_untracked(value.as_ref())
                && !*disabled.get()
            {
                let _ = element.focus();
            }
        }
    });

    let selected = create_ref(cx, move || properties.is_selected(value.as_ref()));
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

    view! { cx,
        li(
            on:keydown = on_key_down, on:click = on_click, on:focus = on_focus, on:blur = on_blur,
            data-sh = "listbox-option", data-sh-owner = context.owner_id, role = "option", tabindex = -1,
            ref = node, disabled = *disabled.get(), aria-selected = selected(), class = class,
            data-sh-selected = selected(), data-sh-active = properties.is_active(value.as_ref()),
            ..props.attributes
        ) {
            (children)
        }
    }
}
