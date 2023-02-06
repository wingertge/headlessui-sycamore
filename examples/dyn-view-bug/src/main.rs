use sycamore::prelude::*;

struct Context(RcSignal<bool>);

#[component(inline_props)]
fn Wrapper<'cx, G: Html>(
    cx: Scope<'cx>,
    children: Children<'cx, G>,
    show: RcSignal<bool>,
) -> View<G> {
    let context = create_ref(cx, Context(show));
    let mut children = Some(children);

    View::new_dyn_scoped(cx, move |cx| {
        let mut view = None;
        if let Some(children) = children.take() {
            provide_context_ref(cx, context);
            view = Some(children.call(cx))
        }
        view.unwrap()
    })
}

#[component]
fn Inner<G: Html>(cx: Scope) -> View<G> {
    let context: &Context = use_context(cx);

    View::new_dyn(cx, move || {
        context.0.track();
        View::empty()
    })
}

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let show = create_rc_signal(false);

    view! { cx,
        button(on:click = {let show = show.clone(); move |_| show.set(true) }) { "Show" }
        Wrapper(show = show.clone()) {
            Inner
        }
    }
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    sycamore::render(|cx| view! { cx, App {} })
}
