// Copyright 2025 Olivier Faure
// SPDX-License-Identifier: MIT

mod app_logic;
mod graph;
mod process_graph;

fn main() {
    use app_logic::{AppState, app_logic};
    use xilem_web::{App, document_body};

    console_error_panic_hook::set_once();
    App::new(document_body(), AppState::initial(), app_logic).run();
}
