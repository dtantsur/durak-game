// Copyright 2018 Dmitry Tantsur <divius.inside@gmail.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Durak card game 2x2.
//!
//! See [wikipedia](https://en.wikipedia.org/wiki/Durak) for a game
//! explanation. This package implements the simpest variant as a CLI
//! application.

// NOTE: we do not use generic deny(warnings) to avoid breakages with new
// versions of the compiler. Add more warnings here as you discover them.
// Taken from https://github.com/rust-unofficial/patterns/
#![deny(const_err,
        // dead_code,
        improper_ctypes,
        legacy_directory_ownership,
        missing_copy_implementations,
        missing_debug_implementations,
        non_shorthand_field_patterns,
        no_mangle_generic_items,
        overflowing_literals,
        path_statements ,
        patterns_in_fns_without_body,
        plugin_as_library,
        private_in_public,
        private_no_mangle_fns,
        private_no_mangle_statics,
        safe_extern_statics,
        trivial_casts,
        trivial_numeric_casts,
        unconditional_recursion,
        unions_with_drop_fields,
        unsafe_code,
        // unused,
        unused_allocation,
        unused_comparisons,
        unused_doc_comments,
        unused_extern_crates,
        unused_import_braces,
        unused_parens,
        unused_qualifications,
        unused_results,
        while_true)]

extern crate rand;
extern crate termion;

mod ai;
mod card;
mod game;
mod ui;

use std::io;

use termion::raw::IntoRawMode;

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout().into_raw_mode()
        .expect("Cannot move stdout to raw mode");
    let g = game::Game::new(ai::AI::new());
    let mut u = ui::Ui::new(g, stdin, stdout);
    u.start().expect("Game crashed");
}
