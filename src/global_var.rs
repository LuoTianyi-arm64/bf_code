use std::cell::RefCell;
use std::collections::HashMap;

thread_local! {
    pub static GLOBAL_VEC_IS_USED: RefCell<Vec<usize>> = RefCell::new(Vec::new());
    pub static GLOBAL_VAR_HASHMAP_U8: RefCell<HashMap<String, bf_u8>> = RefCell::new(HashMap::new());
}

#[derive(Debug, Clone, Copy)]
pub struct bf_u8 (pub u8, pub usize);

pub fn with_vec<F, R>(f: F) -> R
where F: FnOnce(&mut Vec<usize>) -> R {
    GLOBAL_VEC_IS_USED.with(|v| f(&mut v.borrow_mut()))
}

pub fn with_vec_borrow<F, R>(f: F) -> R
where F: FnOnce(&Vec<usize>) -> R {
    GLOBAL_VEC_IS_USED.with(|v| f(&v.borrow()))
}

pub fn with_HashMap_u8<F, R>(f: F) -> R
where F: FnOnce(&mut HashMap<String, bf_u8>) -> R {
    GLOBAL_VAR_HASHMAP_U8.with(|v| f(&mut v.borrow_mut()))
}

pub fn with_HashMap_u8_borrow<F, R>(f: F) -> R
where F: FnOnce(&HashMap<String, bf_u8>) -> R {
    GLOBAL_VAR_HASHMAP_U8.with(|v| f(&v.borrow()))
}
