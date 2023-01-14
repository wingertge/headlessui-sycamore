use std::mem;

use sycamore::prelude::*;
use sycamore_utils::ReactiveBool;

pub struct HeadlessToggleContext<'cx> {
    pub checked: &'cx Signal<bool>,
    pub disabled: ReactiveBool<'cx>,
}

pub fn use_headless_toggle<'cx>(
    checked: &'cx Signal<bool>,
    disabled: ReactiveBool<'cx>,
) -> HeadlessToggleContext<'static> {
    HeadlessToggleContext::<'static> {
        checked: unsafe { mem::transmute(checked) },
        disabled: unsafe { mem::transmute(disabled) },
    }
}
