use sycamore::{
    prelude::{Attributes, Children},
    web::Html,
    Props,
};
use sycamore_utils::ReactiveStr;

mod checkbox;
mod disclosure;
mod listbox;
mod menu;
mod radio_group;
mod select;
mod toggle;

pub use checkbox::*;
pub use disclosure::*;
pub use listbox::*;
pub use menu::*;
pub use radio_group::*;
pub use select::*;
pub use toggle::*;

#[derive(Props)]
pub struct BaseProps<'cx, G: Html> {
    #[prop(default, setter(into))]
    class: ReactiveStr<'cx>,
    children: Children<'cx, G>,
    attributes: Attributes<'cx, G>,
}
