use std::cell::RefCell;

use gloo_timers::callback::Timeout;
use sycamore::{builder::prelude::div, prelude::*, rt::JsCast, web::html::ev};
use sycamore_utils::{DynamicElement, ReactiveStr};
use web_sys::{HtmlElement, KeyboardEvent};

use crate::{
    hooks::create_id,
    utils::{
        as_static, class, focus_navigator::FocusNavigator, get_ref, scoped_children, SetDynAttr,
    },
};

#[derive(Props)]
pub struct MenuProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn Menu<'cx, G: Html>(cx: Scope<'cx>, props: MenuProps<'cx, G>) -> View<G> {
    let id = create_id();
    let focus_ref = get_ref(cx, &props.attributes);
    let focus_nav = FocusNavigator::<G>::new(id.clone(), focus_ref);

    let children = scoped_children(cx, props.children, {
        let id = id.clone();
        move |cx| {
            provide_context(cx, FocusNavigator::<G>::new(id, focus_ref));
        }
    });

    props.attributes.exclude_keys(&["id", "role", "ref"]);
    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    focus_ref.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("id".into(), id.into());
    element.set_attribute("role".into(), "menu".into());
    element.set_attribute("data-sh".into(), "menu".into());
    element.set_attribute("tabindex".into(), "0".into());

    element.event(cx, ev::focus, move |_| {
        focus_nav.set_first_checked();
    });

    view
}

#[derive(Props)]
pub struct MenuItemProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[component]
pub fn MenuItem<'cx, G: Html>(cx: Scope<'cx>, props: MenuItemProps<'cx, G>) -> View<G> {
    let context: &FocusNavigator<'_, G> = as_static(use_context(cx));

    let internal_ref = get_ref(cx, &props.attributes);

    let characters = as_static(create_ref(cx, RefCell::new(String::new())));
    let timeout = as_static(create_ref::<RefCell<Option<Timeout>>>(
        cx,
        RefCell::new(None),
    ));

    on_cleanup(cx, move || {
        if let Some(timeout) = timeout.borrow_mut().take() {
            timeout.cancel();
        }
    });

    let on_key_down = {
        move |e: KeyboardEvent| match e.key().as_str() {
            "ArrowUp" | "ArrowLeft" => {
                e.prevent_default();
                context.set_prev_checked(internal_ref, false);
            }
            "ArrowDown" | "ArrowRight" => {
                e.prevent_default();
                context.set_next_checked(internal_ref, false);
            }
            " " | "Enter" => {
                if let Some(el) = internal_ref
                    .get::<DomNode>()
                    .as_ref()
                    .dyn_ref::<HtmlElement>()
                {
                    el.click();
                }
            }
            "Home" => {
                e.prevent_default();
                context.set_first_checked();
            }
            "End" => {
                e.prevent_default();
                context.set_last_checked();
            }
            key => {
                if key.len() == 1 {
                    characters.borrow_mut().push_str(key);
                    if let Some(timeout) = timeout.borrow_mut().take() {
                        timeout.cancel();
                    }
                    *timeout.borrow_mut() = Some(Timeout::new(100, move || {
                        context.set_first_match(characters.borrow().as_str());
                        characters.borrow_mut().clear();
                    }));
                }
            }
        }
    };

    let children = props.children.call(cx);
    props
        .attributes
        .exclude_keys(&["data-sh-owner", "role", "tabindex", "ref", "on:keydown"]);
    let class = class(cx, &props.attributes, props.class);

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    internal_ref.set(element.clone());

    element.set_dyn_attr(cx, "class", move || class.to_string());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    element.set_attribute("data-sh-owner".into(), context.owner_id.clone().into());
    element.set_attribute("role".into(), "menuitem".into());
    element.set_attribute("data-sh".into(), "menu-item".into());
    element.set_attribute("tabindex".into(), "-1".into());

    element.event(cx, ev::keydown, on_key_down);

    view
}
