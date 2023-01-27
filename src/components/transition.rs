use std::borrow::Cow;
#[cfg(target_arch = "wasm32")]
use std::mem;

#[cfg(target_arch = "wasm32")]
use crate::utils::oneshot_event;
use crate::utils::{get_ref, SetDynAttr};
#[cfg(target_arch = "wasm32")]
use sycamore::web::html::ev;
use sycamore::{builder::prelude::div, prelude::*};
use sycamore_utils::{DynamicElement, ReactiveBool, ReactiveStr};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::Closure, JsCast};
#[cfg(target_arch = "wasm32")]
use web_sys::Window;

pub type TransitionProp<'cx, G> =
    Box<dyn FnOnce(BoundedScope<'_, 'cx>, &'cx ReadSignal<bool>) -> View<G> + 'cx>;

#[allow(unused)]
#[derive(Props)]
pub struct TransitionProps<'cx, G: Html> {
    show: ReactiveBool<'cx>,

    #[prop(default, setter(into))]
    enter: Cow<'static, str>,
    #[prop(default, setter(into))]
    enter_from: Cow<'static, str>,
    #[prop(default, setter(into))]
    enter_to: Cow<'static, str>,
    #[prop(default, setter(into))]
    entered: Cow<'static, str>,
    #[prop(default, setter(into))]
    leave: Cow<'static, str>,
    #[prop(default, setter(into))]
    leave_from: Cow<'static, str>,
    #[prop(default, setter(into))]
    leave_to: Cow<'static, str>,

    #[prop(setter(into))]
    before_enter: Option<Box<dyn Fn()>>,
    #[prop(setter(into))]
    after_enter: Option<Box<dyn Fn()>>,
    #[prop(setter(into))]
    before_leave: Option<Box<dyn Fn()>>,
    #[prop(setter(into))]
    after_leave: Option<Box<dyn Fn()>>,

    #[prop(default = div.into(), setter(into))]
    element: DynamicElement<'cx, G>,
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}

#[cfg(target_arch = "wasm32")]
struct Properties {
    enter: Cow<'static, str>,
    enter_from: Cow<'static, str>,
    enter_to: Cow<'static, str>,
    entered: Cow<'static, str>,
    leave: Cow<'static, str>,
    leave_from: Cow<'static, str>,
    leave_to: Cow<'static, str>,

    before_enter: Option<Box<dyn Fn()>>,
    after_enter: Option<Box<dyn Fn()>>,
    before_leave: Option<Box<dyn Fn()>>,
    after_leave: Option<Box<dyn Fn()>>,
}

pub struct TransitionContext<G: Html> {
    pub node_ref: &'static NodeRef<G>,
}

#[component]
pub fn Transition<'cx, G: Html>(cx: Scope<'cx>, props: TransitionProps<'cx, G>) -> View<G> {
    let visible = create_signal(cx, props.show.get());
    #[cfg(target_arch = "wasm32")]
    let properties = create_ref(
        cx,
        Properties {
            enter: props.enter,
            enter_from: props.enter_from,
            enter_to: props.enter_to,
            entered: props.entered,
            leave: props.leave,
            leave_from: props.leave_from,
            leave_to: props.leave_to,

            before_enter: props.before_enter,
            after_enter: props.after_enter,
            before_leave: props.before_leave,
            after_leave: props.after_leave,
        },
    );

    let node = get_ref(cx, &props.attributes);

    #[cfg(target_arch = "wasm32")]
    let mut initial = true;

    #[cfg(target_arch = "wasm32")]
    let mut transition = move |element: &DomNode, should_enter: bool, window: Window| {
        if should_enter {
            if initial {
                let Properties {
                    enter,
                    enter_from,
                    enter_to,
                    entered,
                    before_enter,
                    after_enter,
                    ..
                } = properties;

                let end_transition = move || {
                    element.remove_class(enter);
                    element.remove_class(enter_to);
                    element.add_class(entered);
                    if let Some(after_enter) = after_enter.as_ref() {
                        after_enter();
                    }
                };

                if let Some(before_enter) = before_enter.as_ref() {
                    before_enter();
                }

                element.add_class(enter);
                element.add_class(enter_from);

                let _ = window.request_animation_frame({
                    let element = element.clone();
                    let enter_from = enter_from.clone();
                    let enter_to = enter_to.clone();

                    let boxed: Box<dyn Fn()> = Box::new(move || {
                        element.remove_class(&enter_from);
                        element.add_class(&enter_to);
                        oneshot_event(&element, ev::transitionend, |_| end_transition());
                        oneshot_event(&element, ev::animationend, |_| end_transition());
                    });

                    Closure::<dyn Fn()>::new::<Box<dyn Fn() + 'static>>(unsafe {
                        mem::transmute(boxed)
                    })
                    .as_ref()
                    .unchecked_ref()
                });

                initial = false;
            }
        } else {
            let Properties {
                leave,
                leave_from,
                leave_to,
                entered,
                before_leave,
                after_leave,
                ..
            } = properties;

            if let Some(before_leave) = before_leave {
                before_leave();
            }

            element.remove_class(entered);
            element.add_class(leave);
            element.add_class(leave_from);

            let _ = window.request_animation_frame({
                let leave_from = leave_from.clone();
                let leave_to = leave_to.clone();
                let element = element.clone();

                Closure::<dyn Fn()>::new(move || {
                    element.remove_class(&leave_from);
                    element.add_class(&leave_to);
                })
                .as_ref()
                .unchecked_ref()
            });

            let end_transition = move || {
                element.remove_class(leave);
                element.remove_class(leave_to);
                visible.set(false);
                if let Some(after_leave) = after_leave {
                    after_leave();
                }
            };
            oneshot_event(element, ev::transitionend, move |_| end_transition());
            oneshot_event(element, ev::animationend, move |_| end_transition());
        }
    };

    create_effect(cx, move || {
        let should_show = props.show.get();

        if should_show {
            visible.set(true);
        }

        #[cfg(target_arch = "wasm32")]
        {
            if let Some((window, node)) = web_sys::window().zip(node.try_get::<DomNode>()) {
                transition(&node, should_show, window);
            }
        }
    });

    let view = props.element.call(cx);
    let element = view.as_node().unwrap();

    node.set(element.clone());

    if let Some(context) = try_use_context::<TransitionContext<G>>(cx) {
        context.node_ref.set(element.clone());
    }

    let class = class!(cx, props);
    let children = props.children.call(cx);

    element.set_class_name((*class.get()).clone().into());
    element.set_children(cx, children);
    element.apply_attributes(cx, &props.attributes);

    view! { cx,
        (if *visible.get() {
            view.clone()
        } else {
            View::empty()
        })
    }
}
