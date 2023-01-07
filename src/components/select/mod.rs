use sycamore::prelude::*;

use crate::utils::DynBool;

pub struct HeadlessSelectSingleOptions<'cx, T> {
    pub value: &'cx Signal<T>,
    pub disabled: DynBool,
}

pub struct HeadlessSelectProperties<'cx, T: PartialEq + 'static> {
    active: RcSignal<Option<T>>,
    value: &'cx Signal<T>,
    disabled: DynBool,
}

impl<'cx, T: PartialEq> HeadlessSelectProperties<'cx, T> {
    pub fn is_selected<'a: 'cx>(&'a self, cx: Scope<'a>, value: &'a T) -> &'cx ReadSignal<bool> {
        create_selector(cx, move || self.value.get().as_ref() == value)
    }

    pub fn select(&self, value: T) {
        self.value.set(value);
    }

    pub fn disabled(&self) -> bool {
        (self.disabled)()
    }

    pub fn has_active(&self) -> bool {
        self.active.get().is_some()
    }

    pub fn is_active(&self, value: &T) -> bool {
        self.active
            .get()
            .as_ref()
            .as_ref()
            .map(|active| active == value)
            .unwrap_or(false)
    }

    pub fn focus(&self, value: T) {
        self.active.set(Some(value));
    }

    pub fn blur(&self) {
        self.active.set(None);
    }
}

pub fn use_headless_select_single<'cx, T: PartialEq>(
    options: HeadlessSelectSingleOptions<'static, T>,
) -> HeadlessSelectProperties<'static, T> {
    let HeadlessSelectSingleOptions { value, disabled } = options;
    HeadlessSelectProperties {
        active: create_rc_signal(None),
        value,
        disabled,
    }
}
