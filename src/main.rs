// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64

use bf_code::*;
use bf_asm::*;

fn main() {
    let mut bf_code = String::new();
    bf_code.push_str(&echo("_KOSHINO_ is a pig."));
    simplify_bf!(code bf_code, target bf_code);
    println!("{bf_code}");
}
