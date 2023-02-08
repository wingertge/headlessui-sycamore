use sycamore::{futures::spawn_local, prelude::*};

#[component]
fn App<G: Html>(cx: Scope) -> View<G> {
    let show = create_rc_signal(false);

    // Update without the need for manual interaction
    {
        let show = show.clone();
        spawn_local(async move {
            show.set(true);
        });
    }

    View::new_dyn_scoped(cx, move |cx| {
        let show = show.clone();
        View::new_dyn(cx, move || {
            show.track();
            View::empty()
        })
    })
}

pub fn main() {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    wasm_logger::init(wasm_logger::Config::default());

    sycamore::render(|cx| view! { cx, App {} })
}
