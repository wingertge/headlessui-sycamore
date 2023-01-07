use super::{use_headless_toggle, HeadlessToggleContext};
use crate::{
    hooks::create_id,
    utils::{scoped_children, DynBool},
};
use sycamore::prelude::*;
use web_sys::MouseEvent;

#[derive(Props)]
pub struct ToggleProps<'cx, G: Html> {
    checked: &'cx Signal<bool>,
    #[prop(default, setter(into))]
    disabled: DynBool,
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

    props.attributes.exclude_keys(&["on:click", "aria-checked"]);

    view! { cx,
        div(..props.attributes, on:click = on_click, aria-checked = props.checked) {
            (children)
        }
    }
}

#[derive(Props)]
pub struct CheckboxDescriptionProps<'cx, G: Html> {
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn CheckboxDescription<'cx, G: Html>(
    cx: Scope<'cx>,
    props: CheckboxDescriptionProps<'cx, G>,
) -> View<G> {
    props.attributes.exclude_keys(&["id"]);

    let context = use_context::<CheckboxContext>(cx);
    let children = props.children.call(cx);

    view! { cx,
        p(..props.attributes, id = context.description_id) {
            (children)
        }
    }
}

#[derive(Props)]
pub struct CheckboxIndicatorProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: DynBool,
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
    let internal_ref = create_node_ref(cx);

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

    view! { cx,
        button(..props.attributes, ref = internal_ref, id = context.indicator_id, role = "checkbox",
            aria-labelledby = context.label_id, aria-describedby = context.description_id,
            disabled = (state.disabled)(), checked = *state.checked.get(), tabindex = tabindex,
        ) {
            (children)
        }
    }
}

#[derive(Props)]
pub struct CheckboxLabelProps<'cx, G: Html> {
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn CheckboxLabel<'cx, G: Html>(cx: Scope<'cx>, props: CheckboxLabelProps<'cx, G>) -> View<G> {
    let context = use_context::<CheckboxContext>(cx);

    let children = props.children.call(cx);
    props.attributes.exclude_keys(&["id", "for"]);

    view! { cx,
        label(..props.attributes, id = context.label_id, for = context.indicator_id) {
            (children)
        }
    }
}
