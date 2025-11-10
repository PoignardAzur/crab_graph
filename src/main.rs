// Copyright 2025 Olivier Faure
// SPDX-License-Identifier: MIT

#![allow(unused)]

use xilem_web::elements::svg::{g, rect, svg, text};
use xilem_web::interfaces::{Element as _, SvgsvgElement};
use xilem_web::svg::peniko::Color;
use xilem_web::{App, DomFragment, document_body};

struct GroupName(String);

struct NodeId(u64);

struct GraphNode {
    parents: Vec<NodeId>,
    group: GroupName,
    name: String,
}

#[derive(Default)]
struct AppState {}

// ---

const BOX_W: f32 = 140.;
const BOX_H: f32 = 140.;
const HEADER_H: f32 = 28.;
const GAP: f32 = 16.;
const BG_COLOR: &'static str = "#ccc";

fn box_at(row: u32, col: u32) -> impl xilem_web::interfaces::SvggElement<()> + use<> {
    let x = (col as f32) * (BOX_W + GAP) + GAP;
    let y = (row as f32) * (BOX_H + GAP) + GAP;

    g((
        rect(())
            .attr("x", x)
            .attr("y", y)
            .attr("width", BOX_W)
            .attr("height", BOX_H)
            .attr("stroke", "#000")
            .attr("fill", BG_COLOR),
        rect(())
            .attr("x", x)
            .attr("y", y)
            .attr("width", BOX_W)
            .attr("height", HEADER_H)
            .attr("fill", "#000"),
        text("Hello")
            .attr("x", x + 10.0)
            .attr("y", y + HEADER_H * 0.72)
            .attr("fill", "white")
            .attr("font-family", "system-ui, sans-serif")
            .attr("font-size", 14),
    ))
}

fn app_logic(_state: &mut ()) -> impl SvgsvgElement<()> + use<> {
    const WIDTH: f32 = BOX_W * 3. + GAP * 5.;
    const HEIGHT: f32 = BOX_H * 2. + GAP * 3.;
    svg((box_at(0, 0), box_at(0, 1), box_at(0, 2), box_at(1, 0)))
        .attr("width", WIDTH)
        .attr("height", HEIGHT)
        .style(xilem_web::modifiers::style("background", "#eee"))
}

fn main() {
    console_error_panic_hook::set_once();
    App::new(document_body(), (), app_logic).run();
}
