// Copyright 2025 Olivier Faure
// SPDX-License-Identifier: MIT

mod app_logic;
mod graph;

fn main() {
    use xilem_web::{App, document_body};

    console_error_panic_hook::set_once();
    App::new(document_body(), (), app_logic::app_logic).run();
}
