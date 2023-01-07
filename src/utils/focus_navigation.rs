use sycamore::rt::JsCast;
use web_sys::{HtmlElement, Node, NodeList};

fn is_focusable(el: Node) -> bool {
    let el = el.dyn_into::<HtmlElement>().unwrap();
    !el.matches(r#"[data-sh-disabled="true"]"#).unwrap()
}

fn get_next_focusable(nodes: NodeList, anchor: u32, direction: i32) -> Option<Node> {
    let mut current = anchor as i32 + direction;
    while current >= 0 && (current as u32) < nodes.length() {
        if is_focusable(nodes.get(current as u32).unwrap()) {
            return nodes.get(current as u32);
        }
        current += direction;
    }
    None
}

fn get_next_locked_focusable(nodes: NodeList, anchor: u32, direction: i32) -> Option<Node> {
    let mut current = anchor as i32 + direction;
    if direction == 1 && current == nodes.length() as i32 {
        current = 0;
    }
    if direction == -1 && current == -1 {
        current = nodes.length() as i32 - 1;
    }
    while anchor as i32 != current {
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
            if let Some(node) = get_next_focusable(nodes, i, 1) {
                let _ = node.unchecked_into::<HtmlElement>().focus();
            }
            break;
        }
    }
}

pub fn focus_prev_continuous(nodes: NodeList, target_node: &HtmlElement) {
    for i in 0..nodes.length() {
        if &nodes.get(i).unwrap() == target_node.unchecked_ref() && i >= 1 {
            if let Some(node) = get_next_focusable(nodes, i, -1) {
                let _ = node.unchecked_into::<HtmlElement>().focus();
            }
            break;
        }
    }
}

pub fn focus_next(nodes: NodeList, target_node: &HtmlElement) {
    for i in 0..nodes.length() {
        if &nodes.get(i).unwrap() == target_node.unchecked_ref() {
            if let Some(node) = get_next_locked_focusable(nodes, i, 1) {
                let _ = node.unchecked_into::<HtmlElement>().focus();
            }
            break;
        }
    }
}

pub fn focus_prev(nodes: NodeList, target_node: &HtmlElement) {
    for i in 0..nodes.length() {
        if &nodes.get(i).unwrap() == target_node.unchecked_ref() {
            if let Some(node) = get_next_locked_focusable(nodes, i, -1) {
                let _ = node.unchecked_into::<HtmlElement>().focus();
            }
            break;
        }
    }
}
