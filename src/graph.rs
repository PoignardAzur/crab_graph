// Copyright 2025 Olivier Faure
// SPDX-License-Identifier: MIT

#![allow(dead_code)]

#[derive(Default, Debug)]
pub struct Graph {
    pub nodes: Vec<GraphNode>,
    pub dummy_nodes: Vec<DummyNode>,
    pub layers: Vec<Layer>,
    pub edges: Vec<Edge>,
}

#[derive(Debug)]
pub struct GraphNode {
    pub id: NodeId,
    pub name: String,
    pub children: Vec<NodeId>,
    pub layer: u32,
    pub x: f32,
    pub y: f32,
}

#[derive(Debug)]
pub struct DummyNode {
    pub id: NodeId,
    pub destination: NodeId,
    pub layer: u32,
    pub x: f32,
    pub y: f32,
}

#[derive(Default, Debug)]
pub struct Layer {
    pub dummy_nodes: Vec<NodeId>,
    pub nodes: Vec<NodeId>,
}

#[derive(Debug)]
pub struct Edge {
    pub from: NodeId,
    pub to: NodeId,
}

pub type NodeId = i64;

// ---

impl Graph {
    #[track_caller]
    pub fn add_node(&mut self, id: NodeId, name: &str, parents: &[NodeId]) {
        let None = self.nodes.iter().find(|n| n.id == id) else {
            panic!("Graph already has a node with id #{id}");
        };

        for parent_id in parents {
            let Some(parent) = self.nodes.iter_mut().find(|n| n.id == *parent_id) else {
                panic!("Cannot add node with parent #{id}: graph doesn't have a node with that id");
            };
            parent.children.push(id);
            self.edges.push(Edge::new(*parent_id, id));
        }

        self.nodes.push(GraphNode::new(id, name.to_string()));
    }
}

impl GraphNode {
    pub fn new(id: NodeId, name: String) -> Self {
        Self {
            id,
            name,
            children: Vec::new(),
            layer: u32::MAX,
            x: 0.,
            y: 0.,
        }
    }
}

impl Edge {
    pub fn new(from: NodeId, to: NodeId) -> Self {
        Self { from, to }
    }
}
