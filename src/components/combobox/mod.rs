use std::{collections::HashSet, hash::Hash, mem, rc::Rc};

use sycamore::{builder::prelude::div, prelude::*};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};

use crate::{
    hooks::create_id,
    utils::{as_static, class, get_ref, scoped_children, FocusStartPoint, SetDynAttr},
    FocusNavigator,
};

mod button;
mod input;
mod label;
mod options;

pub use button::*;
pub use input::*;
pub use label::*;
pub use options::*;

use super::{DisclosureProperties, SelectProperties, SelectValue};

#[derive(Props)]
pub struct ComboboxProps<'cx, T: Clone + Eq + Hash + 'static, G: Html> {
    #[prop(default)]
    value: Option<&'cx Signal<Option<T>>>,
    #[prop(default)]
    value_multiple: Option<&'cx Signal<HashSet<T>>>,
    #[prop(default)]
    open: Option<&'cx Signal<bool>>,
    #[prop(default)]
    default_open: bool,
    #[prop(default)]
    horizontal: bool,
    #[prop(default)]
    on_disclosure_change: Option<Box<dyn Fn(bool) + 'cx>>,
    #[prop(default)]
    disabled: ReactiveBool<'cx>,
    #[prop(default)]
    toggleable: bool,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

pub struct ComboboxContext {
    multiple: bool,
    owner_id: String,
    label_id: String,
    input_id: String,
    button_id: String,
    options_id: String,
    horizontal: bool,
    hovering: &'static Signal<bool>,
}

#[component]
pub fn Combobox<'cx, T: Clone + Eq + Hash + 'static, G: Html>(
    cx: Scope<'cx>,
    props: ComboboxProps<'cx, T, G>,
) -> View<G> {
    let open = props
        .open
        .unwrap_or_else(|| create_signal(cx, props.default_open));

    let hovering = create_signal(cx, false);
    let owner_id = create_id();
    let label_id = create_id();

    let focus_start = FocusStartPoint::new(cx);
    let context = ComboboxContext {
        multiple: props.value_multiple.is_some(),
        owner_id: owner_id.clone(),
        label_id: label_id.clone(),
        button_id: create_id(),
        options_id: create_id(),
        input_id: create_id(),
        horizontal: props.horizontal,
        hovering: unsafe { mem::transmute(hovering) },
    };

    let active = create_signal::<Option<Rc<T>>>(cx, None);
    let disclosure_properties = DisclosureProperties {
        open: unsafe { mem::transmute(open) },
        disabled: unsafe { mem::transmute(props.disabled.clone()) },
    };
    let properties = if let Some(value) = props.value {
        SelectProperties::<T> {
            value: SelectValue::Single(unsafe { mem::transmute(value) }),
            active: unsafe { mem::transmute(active) },
            disabled: unsafe { mem::transmute(props.disabled.clone()) },
            toggleable: props.toggleable,
        }
    } else if let Some(value) = props.value_multiple {
        SelectProperties::<T> {
            value: SelectValue::Multiple(unsafe { mem::transmute(value) }),
            active: unsafe { mem::transmute(active) },
            disabled: unsafe { mem::transmute(props.disabled.clone()) },
            toggleable: props.toggleable,
        }
    } else {
        return view! { cx, span { "Must provide either 'value' or 'value_multiple'." } };
    };

    let node_ref = get_ref(cx, &props.attributes);
    let children = scoped_children(cx, props.children, |cx| {
        provide_context(cx, context);
        provide_context(
            cx,
            FocusNavigator::new(owner_id.clone(), as_static(node_ref)),
        );
        provide_context(cx, properties);
        provide_context(cx, disclosure_properties);
    });

    create_effect(cx, move || {
        if *open.get() {
            focus_start.save();
        }
        if let Some(on_change) = props.on_disclosure_change.as_ref() {
            on_change(*open.get());
        }
        if !*open.get() {
            focus_start.load();
        }
    });
    let class = class(cx, &props.attributes, props.class);

    props
        .attributes
        .exclude_keys(&["data-sh", "id", "aria-labelledby", "disabled"]);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    node_ref.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);
    element.set_attribute("data-sh".into(), "combobox".into());

    element.set_attribute("id".into(), owner_id.into());
    element.set_attribute("aria-labelledby".into(), label_id.into());
    element.set_dyn_bool(cx, "disabled", move || props.disabled.get());

    view
    /* view! {cx,
        div() { (children) }
    } */
}
