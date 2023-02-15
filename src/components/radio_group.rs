use std::{hash::Hash, mem};
use sycamore::{
    builder::prelude::{div, label},
    component::Attributes,
    prelude::*,
    web::html::ev,
};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
use web_sys::KeyboardEvent;

use crate::{
    hooks::create_id,
    utils::{class, focus_navigator::FocusNavigator, get_ref, scoped_children, SetDynAttr},
};

use super::{use_headless_select_single, HeadlessSelectSingleOptions, SelectProperties};

#[derive(Props)]
pub struct RadioGroupProps<'cx, T, G: Html> {
    value: &'cx Signal<Option<T>>,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

pub struct RadioGroupContext {
    description_id: String,
    label_id: String,
}

#[component]
pub fn RadioGroup<'cx, T: Clone + Eq + Hash + 'static, G: Html>(
    cx: Scope<'cx>,
    props: RadioGroupProps<'cx, T, G>,
) -> View<G> {
    let description_id = create_id();
    let label_id = create_id();
    let internal_ref = get_ref(cx, &props.attributes);

    let context = RadioGroupContext {
        description_id: description_id.clone(),
        label_id: label_id.clone(),
    };
    let select_context = use_headless_select_single::<T>(
        cx,
        HeadlessSelectSingleOptions {
            value: unsafe { mem::transmute(props.value) },
            disabled: unsafe { mem::transmute(props.disabled.clone()) },
            toggleable: false,
        },
    );

    let children = scoped_children(cx, props.children, move |cx| {
        provide_context(
            cx,
            FocusNavigator::new(create_id(), unsafe { &*(internal_ref as *const _) }),
        );
        provide_context(cx, context);
        provide_context(cx, select_context);
    });

    props
        .attributes
        .exclude_keys(&["role", "aria-labelledby", "aria-describedby", "ref"]);
    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    internal_ref.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("role".into(), "radiogroup".into());
    element.set_attribute("aria-labelledby".into(), label_id.into());
    element.set_attribute("aria-describedby".into(), description_id.into());
    element.set_attribute("data-sh".into(), "radio-group".into());

    view
}

#[derive(Props)]
pub struct RadioGroupLabelProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = label.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn RadioGroupLabel<'cx, G: Html>(
    cx: Scope<'cx>,
    props: RadioGroupLabelProps<'cx, G>,
) -> View<G> {
    props.attributes.exclude_keys(&["id"]);
    let children = props.children.call(cx);
    let context = use_context::<RadioGroupContext>(cx);

    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("id".into(), context.label_id.clone().into());
    element.set_attribute("data-sh".into(), "radio-group-label".into());

    view
}

#[derive(Props)]
pub struct RadioGroupDescriptionProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn RadioGroupDescription<'cx, G: Html>(
    cx: Scope<'cx>,
    props: RadioGroupDescriptionProps<'cx, G>,
) -> View<G> {
    props.attributes.exclude_keys(&["id"]);
    let children = props.children.call(cx);
    let context = use_context::<RadioGroupContext>(cx);

    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("id".into(), context.description_id.clone().into());
    element.set_attribute("data-sh".into(), "radio-group-description".into());

    view
}

#[derive(Props)]
pub struct RadioGroupOptionProps<'cx, T: PartialEq, G: Html> {
    value: T,
    #[prop(default, setter(into))]
    disabled: ReactiveBool<'cx>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn RadioGroupOption<'cx, T: Clone + Eq + Hash + 'static, G: Html>(
    cx: Scope<'cx>,
    props: RadioGroupOptionProps<'cx, T, G>,
) -> View<G> {
    let context = use_context::<FocusNavigator<G>>(cx);
    let properties: &SelectProperties<T> = use_context(cx);
    let disabled = &properties.disabled;

    let value = create_ref(cx, props.value);

    let description_id = create_id();
    let label_id = create_id();
    let children = scoped_children(cx, props.children, {
        let label_id = label_id.clone();
        let description_id = description_id.clone();
        |cx| {
            provide_context(
                cx,
                RadioGroupContext {
                    label_id,
                    description_id,
                },
            );
        }
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

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    internal_ref.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("role".into(), "radio".into());
    element.set_attribute("data-sh".into(), "radio-group-option".into());
    element.set_attribute("aria-labelledby".into(), label_id.into());
    element.set_attribute("aria-describedby".into(), description_id.into());
    element.set_attribute("data-sh-owner".into(), context.owner_id.clone().into());
    element.set_dyn_attr(cx, "tabindex", move || tabindex.to_string());
    element.set_dyn_bool(cx, "aria-checked", move || properties.is_selected(value));

    element.event(cx, ev::click, on_click);
    element.event(cx, ev::keydown, on_key_down);
    element.event(cx, ev::focus, on_focus);
    element.event(cx, ev::blur, on_blur);

    view
}
