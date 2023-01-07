use sycamore::{prelude::*, rt::JsCast};
use web_sys::{HtmlElement, NodeList};

use super::focus_navigation::{
    focus_next, focus_next_continuous, focus_prev, focus_prev_continuous,
};

fn query_nodes(el: HtmlElement, owner_id: &str) -> NodeList {
    el.query_selector_all(&format!("[data-sh-owner=\"{owner_id}\"]"))
        .expect("Failed to query nodes")
}

pub struct FocusNavigator<'cx, G: Html> {
    pub owner_id: String,
    internal_ref: &'cx NodeRef<G>,
}

impl<'cx, G: Html> FocusNavigator<'cx, G> {
    pub fn new(owner_id: String, internal_ref: &'static NodeRef<G>) -> Self {
        Self {
            owner_id,
            internal_ref,
        }
    }

    fn query(&self) -> NodeList {
        let internal_ref: HtmlElement = self.internal_ref.get::<DomNode>().unchecked_into();
        query_nodes(internal_ref, &self.owner_id)
    }

    pub fn set_checked(&self, node: &NodeRef<G>) {
        let node: HtmlElement = node.get::<DomNode>().unchecked_into();
        let _ = node.focus();
    }

    pub fn set_next_checked(&self, node: &NodeRef<G>, continuous: bool) {
        if let Some(node) = node.get::<DomNode>().as_ref().dyn_ref::<HtmlElement>() {
            if continuous {
                focus_next_continuous(self.query(), node);
            } else {
                focus_next(self.query(), node);
            }
        }
    }

    pub fn set_prev_checked(&self, node: &NodeRef<G>, continuous: bool) {
        let node = node.get::<DomNode>();
        if let Some(node) = node.as_ref().dyn_ref::<HtmlElement>() {
            if continuous {
                focus_prev_continuous(self.query(), node);
            } else {
                focus_prev(self.query(), node);
            }
        }
    }
}
