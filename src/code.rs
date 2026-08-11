use std::cell::RefCell;
use bf_asm::*;

thread_local! {
    pub static GLOBAL_VEC: RefCell<Vec<usize>> = RefCell::new(Vec::new());
}

#[macro_export]
macro_rules! with_vec {
    ($f:expr) => {
        $crate::code::GLOBAL_VEC.with($f)
    };
}

pub fn push(s: usize) {
    with_vec!(|v| v.borrow_mut().push(s));
}

pub fn pop() -> Option<usize> {
    with_vec!(|v| v.borrow_mut().pop())
}

pub fn len() -> usize {
    with_vec!(|v| v.borrow().len())
}

pub fn is_empty() -> bool {
    with_vec!(|v| v.borrow().is_empty())
}

pub fn clear() {
    with_vec!(|v| v.borrow_mut().clear());
}

pub fn get(index: usize) -> Option<usize> {
    with_vec!(|v| v.borrow().get(index).cloned())
}

pub fn in_vec(value: usize) -> bool {
    with_vec!(|v| v.borrow().iter().any(|&x| x == value))
}

pub fn get_all() -> Vec<usize> {
    with_vec!(|v| v.borrow().clone())
}

pub fn print_all() {
    with_vec!(|v| println!("{:?}", v.borrow()));
}

pub fn echo(text: &str) -> String {
    let temp:Vec<usize> = text.as_bytes().into_iter().map(|x| *x as usize).collect();
    // println!("{temp:?}");
    let mut ptr:usize = 0;
    let mut temp_print = Vec::new();
    loop {
        if !in_vec(ptr) {
            temp_print.push(ptr);
            if temp_print.len() == 2 {
                break;
            }
            ptr += 1;
        }
    }
    let mut bf_code = String::new();
    bf_asm!(mov ram temp_print[0], number temp[0], tmp ram temp_print[1], target bf_code, clean_target_ram false, clean_tmp_ram false);
    bf_asm!(mov output, ram temp_print[0], target bf_code);
    for (i, v) in temp[1..].iter().enumerate() {
        if *v > temp[i] {
            bf_asm!(add ram temp_print[0], number *v - temp[i], tmp ram temp_print[1], target bf_code, clean_tmp_ram false);
        } else if *v < temp[i] {
            bf_asm!(sub ram temp_print[0], number temp[i] - *v, tmp ram temp_print[1], target bf_code, clean_tmp_ram false);
        }
        bf_asm!(mov output, ram temp_print[0], target bf_code);
    }
    bf_code
}
