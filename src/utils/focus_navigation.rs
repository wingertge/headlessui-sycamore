use sycamore::{
    prelude::NodeRef,
    rt::JsCast,
    web::{DomNode, Html},
};
use web_sys::{window, HtmlElement, Node, NodeList};

fn is_focusable(el: Node) -> bool {
    let el = el.dyn_into::<HtmlElement>().unwrap();
    !el.matches(r#"[data-sh-disabled="true"]"#).unwrap()
}

fn get_next_focusable(nodes: &NodeList, anchor: i32, direction: i32) -> Option<Node> {
    let mut current = anchor + direction;
    while current >= 0 && (current as u32) < nodes.length() {
        if is_focusable(nodes.get(current as u32).unwrap()) {
            return nodes.get(current as u32);
        }
        current += direction;
    }
    None
}

fn get_next_locked_focusable(nodes: NodeList, anchor: i32, direction: i32) -> Option<Node> {
    let mut current = anchor + direction;
    if direction == 1 && current == nodes.length() as i32 {
        current = 0;
    }
    if direction == -1 && current == -1 {
        current = nodes.length() as i32 - 1;
    }
    while anchor != current {
        if is_focusable(nodes.get(current as u32).unwrap()) {
            return nodes.get(current as u32);
        }
        current += direction;
        if direction == 1 && current >= nodes.length() as i32 {
            current = 0;
        }
        if direction == -1 && current < 0 {
            current = nodes.length() as i32 - 1;
        }
    }
    None
}

pub fn focus_next_continuous(nodes: NodeList, target_node: &HtmlElement) {
    for i in 0..nodes.length() {
        if &nodes.get(i).unwrap() == target_node.unchecked_ref() && i + 1 < nodes.length() {
            if let Some(node) = get_next_focusable(&nodes, i as i32, 1) {
                let _ = node.unchecked_into::<HtmlElement>().focus();
            }
            break;
        }
    }
}

pub fn focus_prev_continuous(nodes: NodeList, target_node: &HtmlElement) {
    for i in 0..nodes.length() {
        if &nodes.get(i).unwrap() == target_node.unchecked_ref() && i >= 1 {
            if let Some(node) = get_next_focusable(&nodes, i as i32, -1) {
                let _ = node.unchecked_into::<HtmlElement>().focus();
            }
            break;
        }
    }
}

pub fn focus_next(nodes: NodeList, target_node: &HtmlElement) {
    for i in 0..nodes.length() {
        if &nodes.get(i).unwrap() == target_node.unchecked_ref() {
            if let Some(node) = get_next_locked_focusable(nodes, i as i32, 1) {
                let _ = node.unchecked_into::<HtmlElement>().focus();
            }
            break;
        }
    }
}

pub fn focus_prev(nodes: NodeList, target_node: &HtmlElement) {
    for i in 0..nodes.length() {
        if &nodes.get(i).unwrap() == target_node.unchecked_ref() {
            if let Some(node) = get_next_locked_focusable(nodes, i as i32, -1) {
                let _ = node.unchecked_into::<HtmlElement>().focus();
            }
            break;
        }
    }
}

pub fn focus_first(nodes: NodeList) -> bool {
    if nodes.length() > 0 {
        if let Some(node) = get_next_focusable(&nodes, -1, 1)
            .as_ref()
            .and_then(|node| node.dyn_ref::<HtmlElement>())
        {
            let _ = node.focus();
        }
        true
    } else {
        false
    }
}

pub fn focus_last(nodes: NodeList) -> bool {
    if nodes.length() > 0 {
        if let Some(node) = get_next_focusable(&nodes, nodes.length() as i32, 1)
            .as_ref()
            .and_then(|node| node.dyn_ref::<HtmlElement>())
        {
            let _ = node.focus();
        }
        true
    } else {
        false
    }
}

pub fn focus_match(nodes: NodeList, character: &str) {
    let lower = character.to_lowercase();
    for i in 0..nodes.length() {
        if let Some(el) = nodes
            .get(i)
            .as_ref()
            .and_then(|node| node.dyn_ref::<HtmlElement>())
        {
            if el
                .text_content()
                .map(|text| text.to_lowercase().starts_with(&lower))
                .unwrap_or(false)
            {
                let _ = el.focus();
                return;
            }
        }
    }
}

fn as_html_element<G: Html>(node: &NodeRef<G>) -> Option<HtmlElement> {
    node.try_get::<DomNode>()?.to_web_sys().dyn_into().ok()
}

pub fn lock_focus<G: Html>(node: &NodeRef<G>, reverse: bool) -> Option<()> {
    let nodes = get_focusable_elements(node)?;
    let node = as_html_element(node)?;
    let document = window()?.document()?;

    let contains = || {
        node.contains(
            document
                .active_element()
                .as_ref()
                .map(|el| el.dyn_ref().unwrap()),
        )
    };

    if reverse {
        if document.active_element().is_none() || !contains() {
            focus_last(nodes);
        } else {
            focus_prev(nodes, document.active_element()?.dyn_ref()?);
        }
    } else if document.active_element().is_none() || !contains() {
        focus_first(nodes);
    } else {
        focus_next(nodes, document.active_element()?.dyn_ref()?);
    }

    Some(())
}

const QUERY: &str = r#"a[href], area[href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), button:not([disabled]), iframe, object, embed, [tabindex]:not([tabindex="-1"]), [contenteditable]"#;

pub fn get_focusable_elements<G: Html>(node: &NodeRef<G>) -> Option<NodeList> {
    let node: HtmlElement = as_html_element(node)?;
    node.query_selector_all(QUERY).ok()
}
