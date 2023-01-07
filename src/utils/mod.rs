mod calculate_active_index;
pub mod focus_navigation;
pub mod focus_navigator;

use std::{mem, ops::Deref, rc::Rc};

pub use calculate_active_index::*;
use sycamore::prelude::*;

#[derive(Clone)]
pub struct DynBool(Rc<dyn Fn() -> bool + 'static>);

impl DynBool {
    pub fn get(&self) -> bool {
        (self.0)()
    }
}

impl Deref for DynBool {
    type Target = dyn Fn() -> bool;

    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<T: Into<bool> + Clone> From<T> for DynBool {
    fn from(value: T) -> Self {
        let boxed: Rc<dyn Fn() -> bool> = Rc::new(move || value.clone().into());
        DynBool(unsafe { mem::transmute(boxed) })
    }
}

impl Default for DynBool {
    fn default() -> Self {
        Self(Rc::new(|| false))
    }
}

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
