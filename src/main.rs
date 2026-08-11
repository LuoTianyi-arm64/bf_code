// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2026 LuoTianyi-arm64

use bf_code::*;

fn main() {
    let mut bf_code = String::new();
    bf_code.push_str(&echo("Hello,"));
    bf_code.push_str(&echo("World!"));
    println!("{bf_code}");
}
