use std::cell::RefCell;

use gloo_timers::callback::Timeout;
use sycamore::{prelude::*, rt::JsCast};
use web_sys::{HtmlElement, KeyboardEvent};

use crate::{
    hooks::create_id,
    utils::{as_static, class, focus_navigator::FocusNavigator, get_ref, scoped_children},
};

use super::BaseProps;

#[component]
pub fn Menu<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
    let id = create_id();
    let focus_ref = get_ref(cx, &props.attributes);

    let children = scoped_children(cx, props.children, {
        let id = id.clone();
        move |cx| {
            provide_context(cx, FocusNavigator::<G>::new(id, focus_ref));
        }
    });

    props.attributes.exclude_keys(&["id", "role", "ref"]);
    let class = class(cx, &props.attributes, props.class);

    view! { cx,
        div(..props.attributes, id = id, role = "menu", ref = focus_ref, class = class, data-sh = "menu") {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn MenuItem<'cx, G: Html>(cx: Scope<'cx>, props: BaseProps<'cx, G>) -> View<G> {
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

    view! { cx,
        div(
            ..props.attributes, data-sh-owner = context.owner_id, role = "menuitem", tabindex = -1,
            ref = internal_ref, on:keydown = on_key_down, class = class, data-sh = "menu-item"
        ) {
            (children)
        }
    }
}
