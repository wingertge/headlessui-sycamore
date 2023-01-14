use std::{collections::HashSet, hash::Hash, mem, rc::Rc};

use sycamore::prelude::*;
use sycamore_utils::{ReactiveBool, ReactiveStr};

use crate::{
    hooks::create_id,
    utils::{class, scoped_children, FocusStartPoint},
};

mod button;
mod label;
mod options;

pub use button::*;
pub use label::*;
pub use options::*;

use super::{DisclosureProperties, HeadlessSelectProperties, SelectValue};

#[derive(Props)]
pub struct ListBoxProps<'cx, T: Clone + Eq + Hash + 'static, F, G: Html>
where
    F: Fn(bool) + 'cx,
{
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
    on_disclosure_change: Option<F>,
    #[prop(default)]
    disabled: ReactiveBool<'cx>,
    #[prop(default)]
    toggleable: bool,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

pub struct ListboxContext {
    multiple: bool,
    owner_id: String,
    label_id: String,
    button_id: String,
    options_id: String,
    horizontal: bool,
    hovering: &'static Signal<bool>,
}

#[component]
pub fn ListBox<'cx, T: Clone + Eq + Hash + 'static, F: Fn(bool) + 'cx, G: Html>(
    cx: Scope<'cx>,
    props: ListBoxProps<'cx, T, F, G>,
) -> View<G> {
    let open = props
        .open
        .unwrap_or_else(|| create_signal(cx, props.default_open));

    let hovering = create_signal(cx, false);
    let owner_id = create_id();
    let label_id = create_id();
    let button_id = create_id();
    let options_id = create_id();

    let focus_start = FocusStartPoint::new(cx);
    let context = ListboxContext {
        multiple: props.value_multiple.is_some(),
        owner_id: owner_id.clone(),
        label_id: label_id.clone(),
        button_id,
        options_id,
        horizontal: props.horizontal,
        hovering: unsafe { mem::transmute(hovering) },
    };

    let active = create_signal::<Option<Rc<T>>>(cx, None);
    let disclosure_properties = DisclosureProperties {
        open: unsafe { mem::transmute(open) },
        disabled: unsafe { mem::transmute(props.disabled.clone()) },
    };
    let properties = if let Some(value) = props.value {
        HeadlessSelectProperties::<T> {
            value: SelectValue::Single(unsafe { mem::transmute(value) }),
            active: unsafe { mem::transmute(active) },
            disabled: unsafe { mem::transmute(props.disabled.clone()) },
            toggleable: props.toggleable,
        }
    } else if let Some(value) = props.value_multiple {
        HeadlessSelectProperties::<T> {
            value: SelectValue::Multiple(unsafe { mem::transmute(value) }),
            active: unsafe { mem::transmute(active) },
            disabled: unsafe { mem::transmute(props.disabled.clone()) },
            toggleable: props.toggleable,
        }
    } else {
        return view! { cx, span { "Must provide either 'value' or 'value_multiple'." } };
    };

    let children = scoped_children(cx, props.children, |cx| {
        provide_context(cx, context);
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

    view! { cx,
        div(data-sh = "listbox", id = owner_id, aria-labelledby = label_id,
            disabled = props.disabled.get(), class = class
        ) {
            (children)
        }
    }
}
