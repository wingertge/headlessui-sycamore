use super::{use_headless_toggle, HeadlessToggleContext};
use crate::{
    hooks::create_id,
    utils::{class, get_ref, scoped_children, SetDynAttr},
};
use sycamore::{
    builder::prelude::{button, div, label, p},
    prelude::*,
    web::html::ev,
};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
use web_sys::{KeyboardEvent, MouseEvent};

#[derive(Props)]
pub struct ToggleProps<'cx, G: Html> {
    checked: &'cx Signal<bool>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
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
    let context = CheckboxContext {
        label_id: create_id(),
        indicator_id: create_id(),
        description_id: create_id(),
    };

    let children = scoped_children(cx, props.children, move |cx| {
        provide_context(cx, context);
        provide_context(cx, use_headless_toggle(props.checked, props.disabled));
    });

    let on_click = |e: MouseEvent| {
        e.prevent_default();
        props.checked.set(!*props.checked.get_untracked());
    };

    let on_key = |e: KeyboardEvent| {
        match e.key().as_str() {
            " " => {
                e.prevent_default();
                props.checked.set(!*props.checked.get_untracked());
            }
            "Enter" => {
                // Buttons toggle on enter by default. This isn't desired for checkboxes.
                e.prevent_default();
            }
            _ => {}
        }
    };

    props.attributes.exclude_keys(&["on:click"]);
    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh".into(), "checkbox".into());

    element.event(cx, ev::click, on_click);
    element.event(cx, ev::keydown, on_key);

    view
}

#[derive(Props)]
pub struct CheckboxDescriptionProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = p.into(), setter(into))]
    element: DynamicElement<'cx, G>,
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

    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh".into(), "checkbox-description".into());
    element.set_attribute("id".into(), context.description_id.clone().into());

    view
}

#[derive(Props)]
pub struct CheckboxIndicatorProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = button.into(), setter(into))]
    element: DynamicElement<'cx, G>,
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

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    internal_ref.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);
    element.set_attribute("data-sh".into(), "checkbox-indicator".into());

    element.set_attribute("id".into(), context.indicator_id.clone().into());
    element.set_attribute("role".into(), "checkbox".into());
    element.set_attribute("aria-labelledby".into(), context.label_id.clone().into());
    element.set_attribute(
        "aria-describedby".into(),
        context.description_id.clone().into(),
    );
    element.set_dyn_bool(cx, "disabled", move || state.disabled.get());
    element.set_dyn_bool(cx, "checked", move || *state.checked.get());
    element.set_dyn_attr(cx, "tabindex", move || tabindex.to_string());
    element.set_dyn_bool(cx, "aria-checked", move || *state.checked.get());

    view
}

#[derive(Props)]
pub struct CheckboxLabelProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = label.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn CheckboxLabel<'cx, G: Html>(cx: Scope<'cx>, props: CheckboxLabelProps<'cx, G>) -> View<G> {
    let context = use_context::<CheckboxContext>(cx);

    let children = props.children.call(cx);
    props.attributes.exclude_keys(&["id", "for"]);
    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);
    element.set_attribute("data-sh".into(), "checkbox-label".into());

    element.set_attribute("id".into(), context.label_id.clone().into());
    element.set_attribute("for".into(), context.indicator_id.clone().into());

    view
}
