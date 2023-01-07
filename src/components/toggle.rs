use sycamore::prelude::*;

use crate::utils::DynBool;

pub struct HeadlessToggleContext<'cx> {
    pub checked: &'cx Signal<bool>,
    pub disabled: DynBool,
}

pub fn use_headless_toggle<'cx>(
    checked: &'cx Signal<bool>,
    disabled: DynBool,
) -> HeadlessToggleContext<'static> {
    HeadlessToggleContext::<'static> {
        checked: unsafe { &*(checked as *const _) },
        disabled,
    }
}
