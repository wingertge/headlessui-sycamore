use std::cell::RefCell;

use sycamore::{
    reactive::{create_ref, on_cleanup, Scope},
    rt::JsCast,
};
use web_sys::{window, Element, HtmlElement};

#[derive(Default)]
pub struct FocusStartPoint {
    return_element: RefCell<Option<Element>>,
    fsp: RefCell<Option<HtmlElement>>,
}

impl FocusStartPoint {
    pub fn new<'cx>(cx: Scope<'cx>) -> &'cx Self {
        let this = if let Some(document) = window().and_then(|window| window.document()) {
            Self {
                return_element: RefCell::new(document.active_element()),
                fsp: RefCell::new(get_focus_start_point()),
            }
        } else {
            Default::default()
        };
        let this = create_ref(cx, this);
        on_cleanup(cx, move || this.load());
        this
    }

    pub fn load(&self) {
        if let Some(element) = self
            .return_element
            .borrow()
            .as_ref()
            .and_then(|el| el.dyn_ref::<HtmlElement>())
        {
            let _ = element.focus();
        } else {
            set_focus_start_point(self.fsp.borrow().as_ref())
        }
    }

    pub fn save(&self) {
        *self.return_element.borrow_mut() = window()
            .and_then(|window| window.document())
            .and_then(|document| document.active_element());
        *self.fsp.borrow_mut() = get_focus_start_point();
    }
}

fn get_focus_start_point() -> Option<HtmlElement> {
    window()?
        .get_selection()
        .ok()??
        .focus_node()?
        .parent_element()?
        .dyn_into()
        .ok()
}

fn set_focus_start_point(element: Option<&HtmlElement>) {
    if let Some(element) = element {
        let tabindex = element.get_attribute("tabindex");

        let _ = element.set_attribute("tabindex", "-1");
        let _ = element.focus();
        let _ = element.blur();

        if let Some(tabindex) = tabindex {
            let _ = element.set_attribute("tabindex", &tabindex);
        } else {
            let _ = element.remove_attribute("tabindex");
        }
    }
}
