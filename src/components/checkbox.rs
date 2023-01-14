use super::{use_headless_toggle, BaseProps, HeadlessToggleContext};
use crate::{
    hooks::create_id,
    utils::{class, get_ref, scoped_children},
};
use sycamore::prelude::*;
use sycamore_utils::{ReactiveBool, ReactiveStr};
use web_sys::MouseEvent;

#[derive(Props)]
pub struct ToggleProps<'cx, G: Html> {
    checked: &'cx Signal<bool>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

struct CheckboxContext {
    label_id: String,
    indicator_id: String,
    description_id: String,
}

#[component]
pub fn Checkbox<'cx, G: Html>(cx: Scope<'cx>, props: ToggleProps<'cx, G>) -> View<G> {
    let label_id = create_id();
    let indicator_id = create_id();
    let description_id = create_id();

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(
            cx,
            CheckboxContext {
                label_id: label_id.clone(),
                indicator_id: indicator_id.clone(),
                description_id: description_id.clone(),
            },
        );
        provide_context(cx, use_headless_toggle(props.checked, props.disabled));
    });

    let on_click = |e: MouseEvent| {
        e.prevent_default();
        props.checked.set(!*props.checked.get_untracked());
    };

    props.attributes.exclude_keys(&["on:click"]);
    let class = class(cx, &props.attributes, props.class);

    view! { cx,
        div(..props.attributes, class = class, on:click = on_click, data-sh = "checkbox") {
            (children)
        }
    }
}

#[component]
pub fn CheckboxDescription<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    props.attributes.exclude_keys(&["id"]);

    let context = use_context::<CheckboxContext>(cx);
    let children = props.children.call(cx);

    let class = class(cx, &props.attributes, props.class);

    view! { cx,
        p(..props.attributes, class = class, id = context.description_id,
            data-sh = "checkbox-description") {
            (children)
        }
    }
}

#[derive(Props)]
pub struct CheckboxIndicatorProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn CheckboxIndicator<'cx, G: Html>(
    cx: Scope<'cx>,
    props: CheckboxIndicatorProps<'cx, G>,
) -> View<G> {
    let context = use_context::<CheckboxContext>(cx);
    let state = use_context::<HeadlessToggleContext>(cx);
    let internal_ref = get_ref(cx, &props.attributes);

    let children = props.children.call(cx);
    let tabindex = create_selector(cx, move || if props.disabled.get() { -1 } else { 0 });

    props.attributes.exclude_keys(&[
        "ref",
        "id",
        "role",
        "aria-labelledby",
        "aria-describedby",
        "disabled",
        "checked",
        "tabindex",
    ]);
    let class = class(cx, &props.attributes, props.class);

    view! { cx,
        button(..props.attributes, ref = internal_ref, id = context.indicator_id, role = "checkbox",
            aria-labelledby = context.label_id, aria-describedby = context.description_id,
            disabled = state.disabled.get(), checked = *state.checked.get(), tabindex = tabindex,
            aria-checked = state.checked, class = class, data-sh = "checkbox-indicator"
        ) {
            (children)
        }
    }
}

#[component]
pub fn CheckboxLabel<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    let context = use_context::<CheckboxContext>(cx);

    let children = props.children.call(cx);
    props.attributes.exclude_keys(&["id", "for"]);
    let class = class(cx, &props.attributes, props.class);

    view! { cx,
        label(..props.attributes, id = context.label_id, for = context.indicator_id, class = class,
            data-sh = "checkbox-label"
        ) {
            (children)
        }
    }
}
