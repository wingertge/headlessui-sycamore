use std::{hash::Hash, mem, rc::Rc};
use sycamore::{component::Attributes, prelude::*};
use sycamore_utils::{ReactiveBool, ReactiveStr};
use web_sys::KeyboardEvent;

use crate::{
    hooks::create_id,
    utils::{class, focus_navigator::FocusNavigator, get_ref, scoped_children},
};

use super::{use_headless_select_single, BaseProps, HeadlessSelectSingleOptions, SelectProperties};

#[derive(Props)]
pub struct RadioGroupProps<'cx, T, G: Html> {
    value: &'cx Signal<T>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

pub struct RadioGroupContext {
    description_id: String,
    label_id: String,
}

struct RadioGroupValueContext<T: PartialEq + 'static> {
    value: &'static Signal<T>,
    disabled: ReactiveBool<'static>,
}

#[component]
pub fn RadioGroup<'cx, T: PartialEq + 'static, G: Html>(
    cx: Scope<'cx>,
    props: RadioGroupProps<'cx, T, G>,
) -> View<G> {
    let description_id = create_id();
    let label_id = create_id();
    let internal_ref = get_ref(cx, &props.attributes);

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(
            cx,
            FocusNavigator::new(create_id(), unsafe { &*(internal_ref as *const _) }),
        );
        provide_context(
            cx,
            RadioGroupValueContext::<T> {
                value: unsafe { mem::transmute(props.value) },
                disabled: unsafe { mem::transmute(props.disabled) },
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
    let class = class(cx, &props.attributes, props.class);

    view! { cx,
        div(..props.attributes, class = class, role = "radiogroup", aria-labelledby = label_id,
            aria-describedby = description_id, ref = internal_ref, data-sh = "radio-group"
        ) {
            (children)
        }
    }
}

#[component]
pub fn RadioGroupLabel<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    props.attributes.exclude_keys(&["id"]);
    let children = props.children.call(cx);
    let context = try_use_context::<RadioGroupContext>(cx);

    let class = class(cx, &props.attributes, props.class);

    if let Some(context) = context {
        view! { cx,
            label(..props.attributes, class = class, id = context.label_id, data-sh = "radio-group-label") {
                (children)
            }
        }
    } else {
        view! { cx, "Missing context" }
    }
}

#[component]
pub fn RadioGroupDescription<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    props.attributes.exclude_keys(&["id"]);
    let children = props.children.call(cx);
    let context = use_context::<RadioGroupContext>(cx);

    let class = class(cx, &props.attributes, props.class);

    view! { cx, div(..props.attributes, class = class, id = context.description_id, data-sh = "radio-group-description") {
        (children)
    }}
}

#[derive(Props)]
pub struct RadioGroupOptionProps<'cx, T: PartialEq, G: Html> {
    value: T,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn RadioGroupOption<'cx, T: Eq + Hash + 'static, G: Html>(
    cx: Scope<'cx>,
    props: RadioGroupOptionProps<'cx, T, G>,
) -> View<G> {
    let context = use_context::<FocusNavigator<G>>(cx);
    let RadioGroupValueContext::<T> { value, disabled } = use_context(cx);
    let value = create_memo(cx, move || Some(value.get()));
    let properties: &SelectProperties<T> = create_ref(
        cx,
        use_headless_select_single(
            cx,
            HeadlessSelectSingleOptions {
                value: unsafe { mem::transmute(value) },
                disabled: unsafe { mem::transmute(props.disabled.clone()) },
                toggleable: false,
            },
        ),
    );

    let value = create_ref(cx, Rc::new(props.value));

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

    let internal_ref = get_ref(cx, &props.attributes);
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
    let tabindex = create_memo(cx, move || {
        if *disabled.get() || !properties.is_selected(value) {
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
    let class = class(cx, &props.attributes, props.class);

    view! { cx,
        div(..props.attributes, role = "radio", aria-labelledby = label_id,
            aria-describedby = description_id, ref = internal_ref, on:keydown = on_key_down,
            on:click = on_click, on:focus = on_focus, on:blur = on_blur, tabindex = tabindex,
            data-sh-owner = context.owner_id, aria-checked = properties.is_selected(value), class = class,
            data-sh = "radio-group-option"
        ) {
            (children)
        }
    }
}
