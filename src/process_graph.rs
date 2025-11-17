// Copyright 2025 Olivier Faure
// SPDX-License-Identifier: MIT

use crate::graph::{Graph, Layer};

pub fn process_graph(graph: &mut Graph) {
    for node in &mut graph.nodes {
        if node.layer >= graph.layers.len() as u32 {
            let new_size = node.layer as usize + 1;
            graph.layers.resize_with(new_size, Layer::default);
        }
        graph.layers[node.layer as usize].nodes.push(node.id);
    }
}
