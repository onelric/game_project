// Might or might not be borrowed from the experimental features in macroquad

use std::any::{Any, TypeId};

use std::collections::HashMap;
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
    rc::Rc,
};

static mut STORAGE: Option<HashMap<TypeId, Box<dyn Any>>> = None;

pub fn store<T: Any>(s: T) {
    unsafe {
        if STORAGE.is_none() {
            STORAGE = Some(HashMap::new());
        }

        STORAGE.as_mut().unwrap().insert(TypeId::of::<T>(), Box::new(Rc::new(RefCell::new(s))))
    };
}

pub fn get<T: Any>() -> impl Deref<Target = T> {
    try_get::<T>().unwrap()
}

pub fn try_get<T: Any>() -> Option<impl Deref<Target = T>> {
    unsafe {
        if STORAGE.is_none() {
            STORAGE = Some(HashMap::new());
        }

        STORAGE.as_mut().unwrap().get(&TypeId::of::<T>()).as_ref()
    }
    .and_then(|s| s.downcast_ref::<Rc<RefCell<T>>>().map(|s| s.borrow()))
}

pub fn try_get_mut<T: Any>() -> Option<impl DerefMut<Target = T>> {
    unsafe {
        if STORAGE.is_none() {
            STORAGE = Some(HashMap::new());
        }

        STORAGE.as_mut().unwrap().get(&TypeId::of::<T>()).as_ref()
    }
    .and_then(|s| s.downcast_ref::<Rc<RefCell<T>>>().map(|s| s.borrow_mut()))
}

pub fn get_mut<T: Any>() -> impl DerefMut<Target = T> {
    try_get_mut::<T>().unwrap()
}
