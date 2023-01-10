use std::mem;

use sycamore::{component::Attributes, prelude::*};
use web_sys::KeyboardEvent;

use crate::{
    hooks::create_id,
    utils::{focus_navigator::FocusNavigator, scoped_children, DynBool},
};

use super::{use_headless_select_single, HeadlessSelectSingleOptions};

#[derive(Props)]
pub struct RadioGroupProps<'cx, T, G: Html> {
    value: &'cx Signal<T>,
    #[prop(default, setter(into))]
    disabled: DynBool,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

pub struct RadioGroupContext {
    description_id: String,
    label_id: String,
}

struct RadioGroupValueContext<T: PartialEq + 'static> {
    value: &'static Signal<T>,
    disabled: DynBool,
}

#[component]
pub fn RadioGroup<'cx, T: PartialEq + 'static, G: Html>(
    cx: Scope<'cx>,
    props: RadioGroupProps<'cx, T, G>,
) -> View<G> {
    let description_id = create_id();
    let label_id = create_id();
    let internal_ref = create_node_ref(cx);

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(
            cx,
            FocusNavigator::new(create_id(), unsafe { &*(internal_ref as *const _) }),
        );
        provide_context(
            cx,
            RadioGroupValueContext::<T> {
                value: unsafe { mem::transmute(props.value) },
                disabled: props.disabled,
            },
        );
        provide_context(
            cx,
            RadioGroupContext {
                description_id: description_id.clone(),
                label_id: label_id.clone(),
            },
        );
    });

    props
        .attributes
        .exclude_keys(&["role", "aria-labelledby", "aria-describedby", "ref"]);

    view! { cx,
        div(..props.attributes, role = "radiogroup", aria-labelledby = label_id,
            aria-describedby = description_id, ref = internal_ref) {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn RadioGroupLabel<'cx, G: Html>(
    cx: Scope<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
) -> View<G> {
    attributes.exclude_keys(&["id"]);
    let children = children.call(cx);
    let context = try_use_context::<RadioGroupContext>(cx);

    if let Some(context) = context {
        view! { cx,
            label(..attributes, id = context.label_id) { (children) }
        }
    } else {
        view! { cx, "Missing context" }
    }
}

#[component(inline_props)]
pub fn RadioGroupDescription<'cx, G: Html>(
    cx: Scope<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
) -> View<G> {
    attributes.exclude_keys(&["id"]);
    let children = children.call(cx);
    let context = use_context::<RadioGroupContext>(cx);

    view! { cx, div(..attributes, id = context.description_id) { (children) } }
}

#[derive(Props)]
pub struct RadioGroupOptionProps<'cx, T: PartialEq + Clone, G: Html> {
    value: T,
    #[prop(default, setter(into))]
    disabled: DynBool,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn RadioGroupOption<'cx, T: PartialEq + Clone + 'static, G: Html>(
    cx: Scope<'cx>,
    props: RadioGroupOptionProps<'cx, T, G>,
) -> View<G> {
    let context = use_context::<FocusNavigator<G>>(cx);
    let RadioGroupValueContext { value, disabled } = use_context(cx);
    let properties = create_ref(
        cx,
        use_headless_select_single(HeadlessSelectSingleOptions {
            value,
            disabled: props.disabled.clone(),
        }),
    );

    let value = create_ref(cx, props.value);

    let description_id = create_id();
    let label_id = create_id();
    let children = scoped_children(cx, props.children, |cx| {
        provide_context(
            cx,
            RadioGroupContext {
                label_id: label_id.clone(),
                description_id: description_id.clone(),
            },
        );
    });

    let disabled = create_memo(cx, move || props.disabled.get() || disabled.get());

    let internal_ref = create_node_ref(cx);
    let on_key_down = move |e: KeyboardEvent| {
        if !*disabled.get() {
            match e.key().as_str() {
                "ArrowLeft" | "ArrowUp" => {
                    e.prevent_default();
                    context.set_prev_checked(internal_ref, false);
                }
                "ArrowRight" | "ArrowDown" => {
                    e.prevent_default();
                    context.set_next_checked(internal_ref, false);
                }
                " " | "Enter" => {
                    context.set_checked(internal_ref);
                }
                _ => {}
            }
        }
    };
    let on_click = move |_| {
        if !*disabled.get() {
            properties.select(value.clone());
        }
    };
    let on_focus = {
        move |_| {
            if !*disabled.get() {
                properties.focus(value.clone());
                properties.select(value.clone());
            }
        }
    };
    let on_blur = move |_| {
        if !*disabled.get() {
            properties.blur();
        }
    };
    let selected = properties.is_selected(cx, value);
    let tabindex = create_memo(cx, move || {
        if *disabled.get() || !*selected.get() {
            -1
        } else {
            0
        }
    });
    props.attributes.exclude_keys(&[
        "role",
        "aria-labelledby",
        "aria-describedby",
        "ref",
        "on:keydown",
        "on:click",
        "on:focus",
        "on:blur",
        "tabindex",
        "data-sh-owner",
        "aria-checked",
    ]);

    view! { cx,
        div(..props.attributes, role = "radio", aria-labelledby = label_id, aria-describedby = description_id,
            ref = internal_ref, on:keydown = on_key_down, on:click = on_click, on:focus = on_focus,
            on:blur = on_blur, tabindex = tabindex, data-sh-owner = context.owner_id, aria-checked = selected) {
            (children)
        }
    }
}
