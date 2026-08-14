// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64

use crate::global_var::*;
use bf_asm::*;



pub fn echo(text: &str) -> String {
    let temp:Vec<usize> = text.as_bytes().into_iter().map(|x| *x as usize).collect();
    let mut ptr:usize = 0;
    let mut temp_print = Vec::new();

    loop {
        if !with_vec_borrow(|v| v.iter().any(|&x| x == ptr)) {
            temp_print.push(ptr);
            if temp_print.len() == 2 {
                break;
            }
        }
        ptr += 1;
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
    bf_asm!(mov ram temp_print[0], number 0, target bf_code, clean_target_ram true);
    bf_code
}

pub fn write_var_u8(name: &str, value: u8) -> String {
    let mut bf_code = String::new();
    let mut ptr:usize = 0;
    let temp_print0 = loop {
        if !with_vec_borrow(|v| v.iter().any(|&x| x == ptr)) {
            break ptr;
        }
        ptr += 1;
    };
    match with_HashMap_u8(|v| v.insert(name.to_string(), bf_u8(value, temp_print0))) {
        Some(v) => {

            bf_asm!(add ram v.1, number value as usize, tmp ram temp_print0, target bf_code);
        },
        None => {
            let temp_print1 = loop {
                if !with_vec_borrow(|v| v.iter().any(|&x| x == ptr)) {
                    break ptr;
                }
                ptr += 1;
            };
            bf_asm!(add ram temp_print0, number value as usize, tmp ram temp_print1, target bf_code);
            with_vec(|v| v.push(temp_print0));
        },
    }

    // println!("{:?}", with_HashMap_u8(|v| v.clone()));
    bf_code
}

pub fn echo_var_u8(name: &str) -> String {
    let mut bf_code = String::new();
    let mut temp_print = Vec::new();
    let mut ptr:usize = 0;

    loop {
        if !with_vec_borrow(|v| v.iter().any(|&x| x == ptr)) {
            temp_print.push(ptr);
            if temp_print.len() == 2 {
                break;
            }
        }
        ptr += 1;
    }
    match with_HashMap_u8(|v| v.get(name).copied()) {
        Some(v) => {
            bf_asm!(mov output, ram v.1, target bf_code);
        },
        None => panic!("未定义 {name} 变量"),
    }
    bf_code
}

