// Copyright 2025 Olivier Faure
// SPDX-License-Identifier: MIT

#![allow(unused)]

use xilem_web::elements::svg::{g, rect, svg, text};
use xilem_web::interfaces::{Element as _, SvgsvgElement};
use xilem_web::svg::peniko::Color;
use xilem_web::{App, DomFragment, document_body};

use crate::graph::Graph;
use crate::process_graph::{add_dummy_nodes, fill_layers, layer_graph};

pub struct EdgePath {
    pub path: Vec<(f32, f32)>,
}

#[derive(Default)]
pub struct AppState {
    graph: Graph,
}

impl AppState {
    pub fn initial() -> Self {
        let mut graph = Graph::default();
        graph.add_node(0, "hello", &[]);
        graph.add_node(1, "foo", &[]);
        graph.add_node(2, "bar", &[1]);
        graph.add_node(3, "stuff", &[2]);
        graph.add_node(4, "stuff", &[2]);

        layer_graph(&mut graph);
        add_dummy_nodes(&mut graph, 32);
        fill_layers(&mut graph);

        Self { graph }
    }
}

// ---

const BOX_W: f32 = 140.;
const BOX_H: f32 = 140.;
const HEADER_H: f32 = 28.;
const GAP: f32 = 16.;
const BG_COLOR: &'static str = "#ccc";

fn box_at(
    name: String,
    row: usize,
    col: usize,
) -> impl xilem_web::interfaces::SvggElement<AppState> + use<> {
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
        text(name)
            .attr("x", x + 10.0)
            .attr("y", y + HEADER_H * 0.72)
            .attr("fill", "white")
            .attr("font-family", "system-ui, sans-serif")
            .attr("font-size", 14),
    ))
}

pub fn app_logic(state: &mut AppState) -> impl SvgsvgElement<AppState> + use<> {
    let layer_count = state.graph.layers.len();
    let column_count = state
        .graph
        .layers
        .iter()
        .map(|layer| layer.nodes.len())
        .max()
        .unwrap();

    let canvas_width = BOX_W * column_count as f32 + GAP * (column_count + 2) as f32;
    let canvas_height = BOX_H * layer_count as f32 + GAP * (layer_count + 2) as f32;

    let layers = state
        .graph
        .layers
        .iter()
        .enumerate()
        .map(|(row, layer)| {
            layer
                .nodes
                .iter()
                .enumerate()
                .map(|(col, node_id)| box_at(state.graph.node(*node_id).name.clone(), row, col))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    svg(layers)
        .attr("width", canvas_width)
        .attr("height", canvas_height)
        .style(xilem_web::modifiers::style("background", "#eee"))
}
