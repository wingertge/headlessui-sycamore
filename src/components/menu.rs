use std::{cell::RefCell, sync::Arc, time::Duration};

use fluvio_wasm_timer::Delay;
use sycamore::{prelude::*, rt::JsCast};
use web_sys::{HtmlElement, KeyboardEvent};

use crate::{
    hooks::create_id,
    utils::{focus_navigator::FocusNavigator, scoped_children},
};

#[component(inline_props)]
pub fn Menu<'cx, G: Html>(
    cx: Scope<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
) -> View<G> {
    let id = create_id();
    let focus_ref = create_node_ref(cx);

    let children = scoped_children(cx, children, {
        let id = id.clone();
        move |cx| {
            provide_context(cx, FocusNavigator::<G>::new(id, focus_ref));
        }
    });

    attributes.exclude_keys(&["id", "role", "ref"]);

    view! { cx,
        div(..attributes, id = id, role = "menu", ref = focus_ref) {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn MenuItem<'cx, G: Html>(
    cx: Scope<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
) -> View<G> {
    let context: &FocusNavigator<'_, G> = use_context(cx);

    let internal_ref = create_node_ref(cx);

    let characters = Arc::new(RefCell::new(String::new()));
    let mut delay: Option<Arc<RefCell<Delay>>> = None;

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
                    characters.as_ref().borrow_mut().push_str(key);
                    if let Some(delay) = delay.clone() {
                        delay
                            .as_ref()
                            .borrow_mut()
                            .reset(Duration::from_millis(100));
                    } else {
                        delay = Some(Arc::new(RefCell::new(Delay::new(Duration::from_millis(
                            100,
                        )))));
                        let context = context.clone();
                        let delay = delay.clone();
                        let characters = characters.clone();
                        wasm_bindgen_futures::spawn_local(async move {
                            if let Some(delay) = delay {
                                let mut delay = delay.as_ref().borrow_mut();
                                let _ = (&mut *delay).await;
                            }
                            context.set_first_match(characters.borrow().as_str());
                            characters.as_ref().borrow_mut().clear();
                        });
                    }
                }
            }
        }
    };

    let children = children.call(cx);
    attributes.exclude_keys(&["data-sh-owner", "role", "tabindex", "ref", "on:keydown"]);

    view! { cx,
        div(
            ..attributes, data-sh-owner = context.owner_id, role = "menuitem", tabindex = -1,
            ref = internal_ref, on:keydown = on_key_down
        ) {
            (children)
        }
    }
}
