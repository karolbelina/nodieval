use bevy::prelude::*;
use base::HexCoord;
use std::f32::consts::PI;
use bevy::render::{mesh::Indices, pipeline::PrimitiveTopology};

pub fn hex_to_point(size: f32, hex_coord: &HexCoord<isize>) -> Vec2 {
    let x = size * (3_f32.sqrt() * hex_coord.q() as f32 + 3_f32.sqrt() / 2_f32 * hex_coord.r() as f32);
    let y = size * (3_f32 / 2_f32 * hex_coord.r() as f32);
    Vec2::new(x, y)
}

pub fn hex_corner_offset(center: &Vec2, size: f32, corner: isize) -> Vec2 {
    let angle = PI / 3_f32 * corner as f32 - PI / 6_f32;
    Vec2::new(center.x + size * angle.cos(), center.y + size * angle.sin())
}

pub fn hex_corners(size: f32, hex_coord: &HexCoord<isize>) -> [Vec2; 6] {
    let mut corners = [Vec2::default(); 6];
    let center = hex_to_point(size, hex_coord);

    for i in 0..6 {
        let offset = hex_corner_offset(&center, size, i as isize);
        corners[i] = Vec2::new(center.x + offset.x, center.y + offset.y);
    }

    corners
}

pub fn generate_hex_mesh() -> Mesh {
    //     3
    // 4       2
    //     0
    // 5       1
    //     6
    let mut pts = [[f32::default(); 3]; 7];
    pts[0] = [0.0, 0.0, 0.0];
    for (i, corner) in hex_corners(1.0, &HexCoord::<isize>::from_qr(0, 0)).iter().enumerate() {
        pts[i + 1] = [corner.x, 0.0, corner.y];
    }
    let indices = vec![0, 2, 1, 0, 3, 2, 0, 4, 3, 0, 5, 4, 0, 6, 5, 0, 1, 6];
    let normals = vec![[0.0, 1.0, 0.0]; 7];
    let uvs = vec![[0.0, 0.0]; 7];

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.set_indices(Some(Indices::U32(indices)));
    mesh.set_attribute(Mesh::ATTRIBUTE_POSITION, pts.to_vec());
    mesh.set_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.set_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh
}
