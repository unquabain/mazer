use svg::{
    Document,
    node::{
        element::{
            Path, Circle, Rectangle,
            path::{Data,Number},
        }
    }
};
use crate::edge::*;
use crate::node::*;
use std::iter::Iterator;

pub trait SpaceRenderer<const DIMS: usize> {
    fn edge_position(&self, edge_id: usize) -> ([f32; DIMS], [f32; DIMS]);
    fn node_position(&self, node_id: usize) -> [f32; DIMS];
}

#[inline]
fn shift(points: ([f32; 2], [f32; 2]), scale: f32) -> ((Number, Number), (Number, Number)) {
    let shift = scale * 0.5;
    let (start, end) = points;

    let (start_x, start_y, end_x, end_y) = (
        start[1], start[0],
        end[1], end[0],
    );

    let mean_x = (start_x + end_x)/2.0;
    let mean_y = (start_y + end_y)/2.0;
    let x_to_y = mean_y - mean_x;
    let y_to_x = mean_x - mean_y;

    let (start_x, start_y, end_x, end_y) = (
        start_y + y_to_x, start_x + x_to_y,
        end_y + y_to_x, end_x + x_to_y,
    );
    let (start_x, start_y, end_x, end_y) = (
        scale*start_x + shift, scale*start_y + shift,
        scale*end_x + shift, scale*end_y + shift,
    );
    ((start_x, start_y), (end_x, end_y))
}

fn walls_2d(space: &impl SpaceRenderer<2>, edges: &[Edge], scale: f32) -> impl Iterator<Item=Path> {
    edges.iter().enumerate()
        .filter_map(move |(eid, e)| {
            match e.direction {
                EdgeDirection::Forward | EdgeDirection::Backward | EdgeDirection::Unknown => return None,
                _ => {}
            }
            let (start, end) = shift(space.edge_position(eid), scale);
            Some(
                Path::new()
                .set("id", format!("wall_{eid}"))
                .set("class", match e.direction {
                    EdgeDirection::Closed => "wall",
                    EdgeDirection::Border => "gateway",
                    _ => "ghost-wall",
                })
                .set("d", 
                    Data::new()
                    .move_to(start)
                    .line_to(end)
                )
            )
        })
}

fn solution_2d(space: &impl SpaceRenderer<2>, edges: &[Edge], scale: f32) -> impl Iterator<Item=Circle> {
    edges.iter().enumerate()
        .filter_map(move |(eid, e)| {
            if e.direction == EdgeDirection::Closed {
                return None
            }
            if !e.solution {
                return None
            }
            let (start, end) = space.edge_position(eid);
            let avg = ((start[0] + end[0])/2.0, (start[1] + end[1])/2.0);
            let avg = ((avg.0 + 1.0) * scale, (avg.1 + 1.0) * scale);
            Some(Circle::new()
                .set("class", "solution")
                .set("cy", avg.0 - 0.5*scale)
                .set("cx", avg.1 - 0.5*scale)
                .set("r", scale * 0.25)
            )
        })
}

fn node(space: &impl SpaceRenderer<2>, node_id: usize, scale: f32, class: impl std::fmt::Display) -> Rectangle {
    let pos = space.node_position(node_id);
    let x = pos[1];
    let y = pos[0];
    let x = scale * x + 0.5 * scale;
    let y = scale * y + 0.5 * scale;
    Rectangle::new()
        .set("class", class.to_string())
        .set("x", x - 0.5 * scale)
        .set("y", y - 0.5 * scale)
        .set("width", 1.0 * scale)
        .set("height", 1.0 * scale)
}

pub fn render_nodes_2d(
    space: &impl SpaceRenderer<2>,
    nodes: &[Node],
    scale: f32,
) -> impl Iterator<Item=Rectangle> {
    nodes.iter().enumerate()
        .map(move |(nid, n)| {
            node(space, nid, scale, format!("node node_group_{}{}", n.group.unwrap_or(usize::MAX), if n.root { " node_root" } else { "" }))
        })
}

pub fn render_svg_2d(
    space: &impl SpaceRenderer<2>,
    edges: &[Edge],
    nodes: &[Node],
    width: usize,
    height: usize,
    scale: f32,
    start: usize,
    end: usize,
) -> Document {
    let mut doc = Document::new()
        .set("viewBox", (-0.5 * scale, -0.5 * scale, (width+2) as f32 *scale, (height+2) as f32*scale))
    ;
    for node in render_nodes_2d(space, nodes, scale) {
        doc = doc.add(node);
    }
    doc = doc.add(node(space, start, scale, "start"))
        .add(node(space, end, scale, "end"));
    for wall in walls_2d(space, edges, scale) {
        doc = doc.add(wall);
    }
    for dot in solution_2d(space, edges, scale) {
        doc = doc.add(dot);
    }

    doc
        .add(
            Rectangle::new()
                .set("class", "wall")
                .set("x", 0)
                .set("y", 0)
                .set("width", width as f32*scale)
                .set("height", height as f32*scale)
        )
}
