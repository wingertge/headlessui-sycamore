use std::mem;
use sycamore::{builder::prelude::div, prelude::*, web::html::ev};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
use web_sys::KeyboardEvent;

use crate::{
    hooks::create_id,
    utils::{as_static, get_ref, scoped_children, SetDynAttr},
    FocusNavigator,
};

use super::TransitionProp;

pub struct TabGroupContext {
    owner_id: String,
    horizontal: bool,
    manual: bool,
}

impl TabGroupContext {
    pub fn id(&self, kind: &str, index: u32) -> String {
        format!("{}__{kind}-{index}", self.owner_id)
    }
}

pub struct TabGroupProperties {
    selected_index: &'static Signal<u32>,
    disabled: ReactiveBool<'static>,
}

#[derive(Props)]
pub struct TabGroupProps<'cx, G: Html> {
    #[prop(default)]
    selected_index: Option<&'cx Signal<u32>>,
    #[prop(default)]
    default_index: u32,
    #[prop(default = true)]
    horizontal: bool,
    #[prop(default)]
    manual: bool,
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
pub fn TabGroup<'cx, G: Html>(cx: Scope<'cx>, props: TabGroupProps<'cx, G>) -> View<G> {
    let owner_id = create_id();

    let selected_index = props
        .selected_index
        .unwrap_or_else(|| create_signal(cx, props.default_index));

    let context = TabGroupContext {
        owner_id: owner_id.clone(),
        horizontal: props.horizontal,
        manual: props.manual,
    };
    let properties = TabGroupProperties {
        selected_index: as_static(selected_index),
        disabled: unsafe { mem::transmute(props.disabled.clone()) },
    };

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(cx, context);
        provide_context(cx, properties);
    });
    let class = class!(cx, props);

    props.attributes.exclude_keys(&["id", "disabled"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("id".into(), owner_id.into());
    element.set_attribute("data-sh".into(), "tab-group".into());
    element.set_dyn_bool(cx, "disabled", move || props.disabled.get());

    view
}

#[derive(Props)]
pub struct TabListProps<'cx, G: Html> {
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn TabList<'cx, G: Html>(cx: Scope<'cx>, props: TabListProps<'cx, G>) -> View<G> {
    let context: &TabGroupContext = use_context(cx);
    let internal_ref = get_ref(cx, &props.attributes);
    let focus = FocusNavigator::new(context.owner_id.clone(), internal_ref);

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(cx, focus);
    });
    let class = class!(cx, props);

    props.attributes.exclude_keys(&["role", "aria-orientation"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    internal_ref.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh".into(), "tab-list".into());
    element.set_attribute("role".into(), "tablist".into());
    element.set_attribute(
        "aria-orientation".into(),
        if context.horizontal {
            "horizontal"
        } else {
            "vertical"
        }
        .into(),
    );

    view
}

#[derive(Props)]
pub struct TabProps<'cx, G: Html> {
    index: u32,
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
pub fn Tab<'cx, G: Html>(cx: Scope<'cx>, props: TabProps<'cx, G>) -> View<G> {
    let context: &TabGroupContext = use_context(cx);
    let focus: &FocusNavigator<G> = use_context(cx);
    let properties: &TabGroupProperties = use_context(cx);

    let node = get_ref(cx, &props.attributes);
    let disabled = create_memo(cx, move || {
        properties.disabled.get() || props.disabled.get()
    });

    let children = props.children.call(cx);
    let class = class!(cx, props);

    props.attributes.exclude_keys(&[
        "role",
        "id",
        "aria-controls",
        "tabindex",
        "disabled",
        "selected",
        "on:keydown",
        "on:click",
        "on:focus",
    ]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    node.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh".into(), "tab".into());
    element.set_attribute("data-sh-owner".into(), context.owner_id.clone().into());
    element.set_attribute("role".into(), "tab".into());
    element.set_attribute("id".into(), context.id("tab", props.index).into());
    element.set_attribute(
        "aria-controls".into(),
        context.id("tab-panel", props.index).into(),
    );
    element.set_dyn_attr(cx, "tabindex", move || {
        if *disabled.get() || *properties.selected_index.get() != props.index {
            "-1"
        } else {
            "0"
        }
    });
    element.set_dyn_bool(cx, "disabled", move || *disabled.get());
    element.set_dyn_bool(cx, "selected", move || {
        *properties.selected_index.get() == props.index
    });

    element.event(cx, ev::keydown, move |e: KeyboardEvent| {
        if !*disabled.get() {
            match (e.key().as_str(), context.horizontal) {
                ("ArrowUp", false) | ("ArrowLeft", true) => {
                    e.prevent_default();
                    focus.set_prev_checked(node, false);
                }
                ("ArrowDown", false) | ("ArrowRight", true) => {
                    e.prevent_default();
                    focus.set_next_checked(node, false);
                }
                (" ", _) | ("Enter", _) => {
                    e.prevent_default();
                    focus.set_checked(node);
                    properties.selected_index.set(props.index);
                }
                ("Home", _) => {
                    e.prevent_default();
                    focus.set_first_checked();
                }
                ("End", _) => {
                    e.prevent_default();
                    focus.set_last_checked();
                }
                _ => {}
            }
        }
    });
    element.event(cx, ev::click, move |_| {
        if !*disabled.get() {
            properties.selected_index.set(props.index);
        }
    });
    element.event(cx, ev::focus, move |_| {
        if !*disabled.get() && !context.manual {
            properties.selected_index.set(props.index);
        }
    });

    view
}

#[derive(Props)]
pub struct TabPanelProps<'cx, G: Html> {
    index: u32,
    #[prop(default)]
    transition: Option<TransitionProp<'cx, G>>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn TabPanel<'cx, G: Html>(cx: Scope<'cx>, props: TabPanelProps<'cx, G>) -> View<G> {
    let context: &TabGroupContext = use_context(cx);
    let properties: &TabGroupProperties = use_context(cx);

    let show = create_selector(cx, move || *properties.selected_index.get() == props.index);

    let children = props.children.call(cx);
    let class = class!(cx, props);

    let apply_props = |element: &G| {
        element.set_dyn_attr(cx, "class", move || class.to_string());
        element.set_children(cx, children);
        element.apply_attributes(cx, &props.attributes);

        element.set_attribute("data-sh".into(), "tab-panel".into());
        element.set_attribute("role".into(), "tabpanel".into());
        element.set_dyn_attr(cx, "tabindex", move || {
            if *properties.selected_index.get() == props.index {
                "0"
            } else {
                "-1"
            }
        });
        element.set_attribute("id".into(), context.id("tab-panel", props.index).into());
        element.set_attribute(
            "aria-labelledby".into(),
            context.id("tab", props.index).into(),
        );
    };

    if let Some(mut transition) = props.transition {
        let view = transition(cx, show);
        if let Some(element) = view.as_node() {
            apply_props(element);
        }
        view
    } else {
        let view = props.element.call(cx);
        let element = view.as_node().unwrap();
        apply_props(element);

        view! { cx,
            (if *show.get() {
                view.clone()
            } else {
                View::empty()
            })
        }
    }
}
