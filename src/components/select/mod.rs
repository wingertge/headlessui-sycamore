use std::{collections::HashSet, hash::Hash, mem, rc::Rc};

use sycamore::prelude::*;
use sycamore_utils::ReactiveBool;

pub struct HeadlessSelectSingleOptions<T: 'static> {
    pub value: &'static Signal<Option<Rc<T>>>,
    pub disabled: ReactiveBool<'static>,
    pub toggleable: bool,
}

pub struct HeadlessSelectProperties<T: Eq + Hash + 'static> {
    pub value: SelectValue<T>,
    pub active: &'static Signal<Option<Rc<T>>>,
    pub disabled: ReactiveBool<'static>,
    pub toggleable: bool,
}

pub enum SelectValue<T: Eq + Hash + 'static> {
    Single(&'static Signal<Option<Rc<T>>>),
    Multiple(&'static Signal<HashSet<Rc<T>>>),
}

impl<T: Eq + Hash + 'static> HeadlessSelectProperties<T> {
    pub fn is_selected(&self, value: &T) -> bool {
        match &self.value {
            SelectValue::Single(selected) => {
                if let Some(selected) = selected.get().as_ref() {
                    selected.as_ref() == value
                } else {
                    false
                }
            }
            SelectValue::Multiple(selected) => selected.get().contains(value),
        }
    }

    pub fn is_selected_untracked(&self, value: &T) -> bool {
        match &self.value {
            SelectValue::Single(selected) => {
                if let Some(selected) = selected.get_untracked().as_ref() {
                    selected.as_ref() == value
                } else {
                    false
                }
            }
            SelectValue::Multiple(selected) => selected.get_untracked().contains(value),
        }
    }

    pub fn select(&self, value: Rc<T>) {
        match &self.value {
            SelectValue::Single(selected) => {
                if self.toggleable && selected.get_untracked().as_ref().as_ref() == Some(&value) {
                    selected.set(None);
                } else {
                    selected.set(Some(value));
                }
            }
            SelectValue::Multiple(selected) => {
                if self.toggleable && selected.get_untracked().contains(&value) {
                    selected.modify().remove(&value);
                } else {
                    selected.modify().insert(value);
                }
            }
        }
    }

    pub fn has_selected(&self) -> bool {
        match &self.value {
            SelectValue::Single(selected) => selected.get().is_some(),
            SelectValue::Multiple(selected) => !selected.get().is_empty(),
        }
    }

    pub fn has_active(&self) -> bool {
        self.active.get().is_some()
    }

    pub fn is_active(&self, value: &T) -> bool {
        if let Some(active) = self.active.get().as_ref() {
            active.as_ref() == value
        } else {
            false
        }
    }

    pub fn focus(&self, value: Rc<T>) {
        self.active.set(Some(value));
    }

    pub fn blur(&self) {
        self.active.set(None);
    }
}

pub fn use_headless_select_single<'cx, T: Hash + Eq>(
    cx: Scope<'cx>,
    options: HeadlessSelectSingleOptions<T>,
) -> HeadlessSelectProperties<T> {
    let HeadlessSelectSingleOptions {
        value,
        disabled,
        toggleable,
    } = options;
    let active = create_signal::<Option<Rc<T>>>(cx, None);
    HeadlessSelectProperties {
        active: unsafe { mem::transmute(active) },
        value: SelectValue::Single(value),
        disabled,
        toggleable,
    }
}
