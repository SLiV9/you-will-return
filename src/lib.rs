//
// Part of you-will-return
// Copyright (c) 2022 Sander in 't Veld
// License: MIT
//

mod wasm4;

#[cfg(feature = "buddy-alloc")]
mod alloc;

use wasm4::*;

#[no_mangle]
fn update()
{
    unsafe { *DRAW_COLORS = 2 }
    text("Hello from Rust!", 10, 10);
}
