// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64

use bf_code::*;
use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_code.push_str(&echo("lty"));
    bf_code.push_str(&write_var_u8(&"l", 108));
    bf_code.push_str(&write_var_u8(&"t", 116));
    bf_code.push_str(&write_var_u8(&"y", 121));
    bf_code.push_str(&echo_var_u8(&"l"));
    bf_code.push_str(&echo_var_u8(&"t"));
    bf_code.push_str(&echo_var_u8(&"y"));
    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}
