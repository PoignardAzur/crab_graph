// Copyright 2025 Olivier Faure
// SPDX-License-Identifier: MIT

use crate::graph::{DummyNode, Edge, Graph, Layer, NodeId};

pub fn layer_graph(graph: &mut Graph) {
    for node in &mut graph.nodes {
        node.layer = 0;
    }

    let all_nodes = graph.nodes.iter().map(|node| node.id).collect::<Vec<_>>();
    for node in all_nodes {
        layer_children_of(graph, node, 0);
    }
}

fn layer_children_of(graph: &mut Graph, parent: NodeId, parent_layer: u32) {
    let layer = parent_layer + 1;

    let children = graph.node(parent).children.clone();
    for child_id in &children {
        let child = graph.node_mut(*child_id);
        if layer > child.layer {
            child.layer = layer;
            layer_children_of(graph, *child_id, layer);
        }
    }
}

// ---

pub fn add_dummy_nodes(graph: &mut Graph, first_dummy_id: NodeId) {
    let edges = std::mem::take(&mut graph.edges);

    let mut new_edges = Vec::new();
    let mut dummy_id = first_dummy_id;

    for edge in edges {
        let from = graph.node(edge.from);
        let to = graph.node(edge.to);

        assert!(to.layer > from.layer, "invalid node order");

        if to.layer > from.layer + 1 {
            let mut prev_id = edge.from;
            for layer in (from.layer + 1)..to.layer {
                graph
                    .dummy_nodes
                    .push(DummyNode::new(dummy_id, edge.to, layer));
                new_edges.push(Edge::new(prev_id, dummy_id));
                prev_id = dummy_id;
                dummy_id += 1;
            }
            new_edges.push(Edge::new(prev_id, edge.to));
        } else {
            new_edges.push(edge);
        }
    }

    graph.edges = new_edges;
}

// ---

pub fn fill_layers(graph: &mut Graph) {
    for node in &mut graph.nodes {
        if node.layer >= graph.layers.len() as u32 {
            let new_size = node.layer as usize + 1;
            graph.layers.resize_with(new_size, Layer::default);
        }
        graph.layers[node.layer as usize].nodes.push(node.id);
    }
}
