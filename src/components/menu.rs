use std::{cell::RefCell, time::Duration};

use fluvio_wasm_timer::Delay;
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
    let context: &FocusNavigator<'_, G> = use_context(cx);

    let internal_ref = get_ref(cx, &props.attributes);

    let characters = create_ref(cx, RefCell::new(String::new()));
    let delay = create_ref::<RefCell<Option<Delay>>>(cx, RefCell::new(None));

    on_cleanup(cx, move || {
        *delay.borrow_mut() = None;
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
                    if let Some(delay) = delay.borrow_mut().as_mut() {
                        delay.reset(Duration::from_millis(100));
                    } else {
                        *delay.borrow_mut() = Some(Delay::new(Duration::from_millis(100)));
                        let delay = as_static(delay);
                        let characters = as_static(characters);
                        let context = as_static(context);
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Some(delay) = delay.borrow_mut().as_mut() {
                                if let Ok(_) = delay.await {
                                    context.set_first_match(characters.borrow().as_str());
                                    characters.borrow_mut().clear();
                                }
                            }
                        });
                    }
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
