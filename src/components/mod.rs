use sycamore::{
    prelude::{Attributes, Children},
    web::Html,
    Props,
};
use sycamore_utils::ReactiveStr;

macro_rules! class {
    ($cx: expr, $props: expr) => {
        $crate::utils::class($cx, &$props.attributes, $props.class)
    };
}

mod checkbox;
mod combobox;
mod dialog;
mod disclosure;
mod listbox;
mod menu;
mod popover;
mod radio_group;
mod select;
mod tabs;
mod toggle;
mod transition;

pub use checkbox::*;
pub use combobox::*;
pub use dialog::*;
pub use disclosure::*;
pub use listbox::*;
pub use menu::*;
pub use popover::*;
pub use radio_group::*;
pub use select::*;
pub use tabs::*;
pub use toggle::*;
pub use transition::*;

#[derive(Props)]
pub struct BaseProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}
