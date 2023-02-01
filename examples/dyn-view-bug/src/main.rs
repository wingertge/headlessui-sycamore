use std::mem;

use sycamore::prelude::*;

struct Context(&'static ReadSignal<bool>);

pub fn scoped_children<'a, G: Html, F>(cx: Scope<'a>, children: Children<'a, G>, f: F) -> View<G>
where
    for<'b> F: FnOnce(Scope<'b>),
{
    let mut view = View::empty();
    create_child_scope(cx, |cx| {
        f(cx);
        view = children.call(cx);
    });
    view
}

#[component(inline_props)]
fn Wrapper<'cx, G: Html>(
    cx: Scope<'cx>,
    children: Children<'cx, G>,
    show: &'cx ReadSignal<bool>,
) -> View<G> {
    let children = scoped_children(cx, children, |cx| {
        provide_context(cx, Context(unsafe { mem::transmute(show) }));
    });

    view! { cx,
        (children)
    }
}

#[component]
fn Inner<G: Html>(cx: Scope) -> View<G> {
    let context: &Context = use_context(cx);

    view! { cx,
        (if *context.0.get() {
            view! { cx, "Test" }
        } else {
            View::empty()
        })
    }
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let show = create_signal(cx, false);

    view! { cx,
        button(on:click = |_| show.set(true)) { "Show" }
        Wrapper(show = show) {
            Inner
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    sycamore::render(|cx| view! { cx, App {} })
}
