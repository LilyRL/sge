use std::{f32, usize};

use bevy_math::Vec2;
use nalgebra::{DMatrix, SymmetricEigen};
use sge_camera::screen_to_world;
use sge_input::last_cursor_pos;
use sge_time::frame_count;

pub struct Network {
    nodes: Vec<Node>,
    last_updated: usize,
    hovered: NodeId,
    node_radius: f32,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId(usize);

pub struct Node {
    id: NodeId,
    pos: Vec2,
    connections: Vec<NodeId>,
}

impl NodeId {
    pub fn get(self, network: &Network) -> &Node {
        network.get(self)
    }

    pub fn get_mut(self, network: &mut Network) -> &mut Node {
        network.get_mut(self)
    }

    pub fn add_connections(&self, network: &mut Network, connections: &[NodeId]) {
        network.add_connections(*self, connections);
    }
}

impl Network {
    pub fn new() -> Self {
        Self {
            nodes: vec![],
            hovered: NodeId(usize::MAX),
            last_updated: usize::MAX,
            node_radius: 20.0,
        }
    }

    pub fn node_radius(&self) -> f32 {
        self.node_radius
    }

    pub fn set_node_radius(&mut self, radius: f32) {
        self.node_radius = radius;
    }

    pub fn with_node_radius(mut self, radius: f32) -> Self {
        self.set_node_radius(radius);
        self
    }

    pub fn calc_positions_by_force(
        &mut self,
        desired_connection_length: f32,
        iteration_count: usize,
    ) {
        let l = desired_connection_length;

        for _ in 0..iteration_count {
            let mut displacements = vec![Vec2::ZERO; self.nodes.len()];

            // push all nodes apart
            for i in 0..self.nodes.len() {
                for j in 0..self.nodes.len() {
                    if i == j {
                        continue;
                    }

                    let delta = self.nodes[i].pos - self.nodes[j].pos;
                    let dist = delta.length().max(0.01);

                    displacements[i] += (delta / dist) * (l / dist);
                }
            }

            // attract connected ndoes
            for (a, b) in self.iter_connections() {
                let delta = self.get(a).pos - self.get(b).pos;
                let dist = delta.length_squared().max(0.01);
                let force = (delta / dist) * (dist / l);
                displacements[a.0] -= force;
                displacements[b.0] += force;
            }

            let temp = l * 0.1;
            for node in self.nodes.iter_mut() {
                let d = displacements[node.id.0];
                let len = d.length().max(0.001);
                node.pos += (d / len) * len.min(temp);
            }
        }
    }

    pub fn calc_positions_laplace(&mut self, desired_connection_length: f32) {
        let n = self.nodes.len();

        if n < 2 {
            return;
        }

        let mut laplacian = DMatrix::<f32>::zeros(n, n);

        for node in &self.nodes {
            for &neighbor in &node.connections {
                let i = node.id.0;
                let j = neighbor.0;

                laplacian[(i, j)] -= 1.0;
                laplacian[(j, i)] -= 1.0;

                laplacian[(i, i)] += 1.0;
                laplacian[(j, j)] += 1.0;
            }
        }

        let eigen = SymmetricEigen::new(laplacian);

        let x_vec = eigen.eigenvectors.column(1);
        let y_vec = eigen.eigenvectors.column(2);

        let edge_count = self.iter_connections().count();
        let scale = if edge_count > 0 {
            let mean_eigen_dist: f32 = self
                .iter_connections()
                .map(|(a, b)| {
                    let dx = x_vec[a.0] - x_vec[b.0];
                    let dy = y_vec[a.0] - y_vec[b.0];
                    (dx * dx + dy * dy).sqrt()
                })
                .sum::<f32>()
                / edge_count as f32;

            if mean_eigen_dist > 1e-6 {
                desired_connection_length / mean_eigen_dist
            } else {
                1.0
            }
        } else {
            1.0
        };

        for node in &mut self.nodes {
            let i = node.id.0;
            node.pos = Vec2::new(x_vec[i] * scale, y_vec[i] * scale);
        }
    }

    fn next_id(&self) -> NodeId {
        NodeId(self.nodes.len())
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn insert_nodes_with_links(&mut self, nodes: &[&[usize]]) {
        let base_n = self.nodes.len();

        for connections in nodes {
            let connections = connections
                .iter()
                .map(|&id| NodeId(id + base_n))
                .collect::<Vec<_>>();
            self.insert(connections);
        }
    }

    pub fn randomize_positions(&mut self) {
        for node in &mut self.nodes {
            node.pos = sge_rng::rand_vec2() * 100.0;
        }
    }

    pub fn nth_node(&self, n: usize) -> Option<NodeId> {
        self.nodes.get(n).map(|node| &node.id).copied()
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    pub fn insert(&mut self, connections: impl Into<Vec<NodeId>>) -> NodeId {
        let id = self.next_id();
        self.nodes.push(Node {
            id: id,
            pos: sge_rng::rand_vec2() * 100.0,
            connections: connections.into(),
        });
        id
    }

    pub fn get(&self, id: NodeId) -> &Node {
        &self.nodes[id.0]
    }

    pub fn get_mut(&mut self, id: NodeId) -> &mut Node {
        &mut self.nodes[id.0]
    }

    pub fn update(&mut self, world: bool) {
        let mut cursor = last_cursor_pos();
        if world {
            cursor = screen_to_world(cursor);
        }
        let radius_squared = self.node_radius * self.node_radius;
        for node in &self.nodes {
            let pos = node.pos;
            let delta = cursor - pos;
            let dist_squared = delta.length_squared();

            if dist_squared < radius_squared {
                self.hovered = node.id;
                return;
            }
        }

        self.hovered = NodeId(usize::MAX);
    }

    pub fn iter_connections<'a>(&'a self) -> ConnectionIterator<'a> {
        ConnectionIterator {
            network: &self,
            node: 0,
            conn: 0,
        }
    }

    pub fn iter_node_positions<'a>(&'a self) -> NodePositionIterator<'a> {
        NodePositionIterator {
            network: &self,
            node: 0,
        }
    }

    pub fn iter_connection_lines<'a>(&'a self) -> ConnectionLineIterator<'a> {
        ConnectionLineIterator {
            network: &self,
            node: 0,
            conn: 0,
        }
    }

    pub fn add_connections(&mut self, id: NodeId, connections: &[NodeId]) {
        self.get_mut(id).connections.extend_from_slice(connections);
    }
}

pub struct ConnectionIterator<'a> {
    network: &'a Network,
    node: usize,
    conn: usize,
}

impl<'a> Iterator for ConnectionIterator<'a> {
    type Item = (NodeId, NodeId);

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.network.nodes.get(self.node)?;

        if let Some(&conn) = node.connections.get(self.conn) {
            self.conn += 1;
            return Some((node.id, conn));
        } else {
            self.node += 1;
            self.conn = 0;
            self.next()
        }
    }
}

pub struct NodePositionIterator<'a> {
    network: &'a Network,
    node: usize,
}

pub struct NodePosition {
    pub pos: Vec2,
    pub id: usize,
    pub is_hovered: bool,
}

impl<'a> Iterator for NodePositionIterator<'a> {
    type Item = NodePosition;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.network.nodes.get(self.node)?;

        self.node += 1;

        Some(NodePosition {
            pos: node.pos,
            id: node.id.0,
            is_hovered: node.id == self.network.hovered,
        })
    }
}

pub struct ConnectionLineIterator<'a> {
    network: &'a Network,
    node: usize,
    conn: usize,
}

pub struct ConnectionLine {
    pub start: Vec2,
    pub end: Vec2,
    pub is_hovered: bool,
}

impl<'a> Iterator for ConnectionLineIterator<'a> {
    type Item = ConnectionLine;

    fn next(&mut self) -> Option<Self::Item> {
        let node = self.network.nodes.get(self.node)?;

        if let Some(&conn) = node.connections.get(self.conn) {
            self.conn += 1;
            return Some(ConnectionLine {
                start: node.pos,
                end: self.network.get(conn).pos,
                is_hovered: node.id == self.network.hovered || conn == self.network.hovered,
            });
        } else {
            self.node += 1;
            self.conn = 0;
            self.next()
        }
    }
}
