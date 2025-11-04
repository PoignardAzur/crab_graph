// Copyright 2025 Olivier Faure
// SPDX-License-Identifier: MIT

use xilem_web::elements::html::div;
use xilem_web::{App, DomFragment, document_body};

#[derive(Default)]
struct AppState {}

fn app_logic(_state: &mut AppState) -> impl DomFragment<AppState> + use<> {
    div("Hello world")
}

fn main() {
    console_error_panic_hook::set_once();
    App::new(document_body(), AppState::default(), app_logic).run();
}
