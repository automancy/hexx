<!-- cargo-sync-readme start -->

 # Hexx

 [![workflow](https://github.com/ManevilleF/hexx/actions/workflows/rust.yml/badge.svg)](https://github.com/ManevilleF/hexx/actions/workflows/rust.yml)
 [![License](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](./LICENSE)
 [![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)
 [![Crates.io](https://img.shields.io/crates/v/hexx.svg)](https://crates.io/crates/hexx)
 [![Docs.rs](https://docs.rs/hexx/badge.svg)](https://docs.rs/hexx)
 [![dependency status](https://deps.rs/crate/hexx/0.4.2/status.svg)](https://deps.rs/crate/hexx)

 Hexagonal tools lib in rust.

 > Inspired by this [`RedBlobGames` article](https://www.redblobgames.com/grids/hexagons/implementation.html).

 This lib allows you to:
 - Manipulate hexagon coordinates
 - Generate hexagonal maps with custom layouts and orientation
 - Generate hexagon meshes (planes or columns)

 I made the choice to use *Axial Coordinates* for performance and utility reasons,
 but the [`Hex`] type has conversion utilities with *cubic*, *doubled* and *offset* coordinates.

 > See the [hexagonal coordinate systems](https://www.redblobgames.com/grids/hexagons/#coordinates)

 ## Features

 `hexx` provides the [`Hex`] coordinates with:
 - Distances
 - Neighbors and directions
 - Lines
 - Ranges
 - Rings
 - Edges
 - Wedges
 - Spirals
 - Rotation
 - Symmetry
 - Vector operations
 - Conversions to other coordinate systems

 And the [`HexMap`] utility, for *wraparound* (seamless) hexagonal maps

 ## Basic usage

```rust
 use hexx::*;

 // Declare points in hexagonal spaces
 let point_a = Hex::new(10, -5);
 let point_b = Hex::new(-8, 15);
 // Find distance between them
 let dist = point_a.unsigned_distance_to(point_b);
 // Compute a line between points
 let line: Vec<Hex> = point_a.line_to(point_b).collect();
 // Compute a ring from `point_a` containing `point_b`
 let ring: Vec<Hex> = point_a.ring(dist);
 // Rotate `point_b` around `point_a` by 2 times 60 degrees clockwise
 let rotated = point_b.rotate_right_around(point_a, 2);
 // Find the direction between the two points
 let dir_a = point_a.direction_to(point_b);
 let dir_b = point_b.direction_to(point_a);
 assert!(dir_a == -dir_b);
 // Compute a wedge from `point_a` to `point_b`
 let wedge = point_a.wedge_to(point_b);
 // Get the average value of the wedge
 let avg = wedge.average();
```

 ## Layout usage

 [`HexLayout`] is the bridge between your world/screen/pixel coordinate system and the hexagonal
 coordinates system.

```rust
 use hexx::*;

 // Define your layout
 let layout = HexLayout {
    hex_size: Vec2::new(1.0, 1.0),
    orientation: HexOrientation::flat(),
    ..Default::default()
 };
 // Get the hex coordinate at the world position `world_pos`.
 let world_pos = Vec2::new(53.52, 189.28);
 let hex = layout.world_pos_to_hex(world_pos);
 // Get the world position of `hex`
 let hex = Hex::new(123, 45);
 let world_pos = layout.hex_to_world_pos(hex);
```

 ## Usage in [Bevy](https://bevyengine.org/) 0.9.x

 If you want to generate 3D hexagonal mesh and use it in [bevy](bevyengine.org) you may do it this way:

```rust
 use bevy::prelude::Mesh;
 use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};
 use hexx::{HexLayout, Hex, MeshInfo};

 pub fn hexagonal_plane(hex_layout: &HexLayout) -> Mesh {
    // Compute hex plane data for at the origin
    let mesh_info = MeshInfo::hexagonal_plane(hex_layout, Hex::ZERO);
    // Compute the bevy mesh
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs.to_vec());
    mesh.set_indices(Some(Indices::U16(mesh_info.indices)));
    mesh
 }
```

 The [`MeshInfo`] type provides the following mesh generations:
 - [`MeshInfo::hexagonal_plane`] (7 vertices) useful for 2D games
 - [`MeshInfo::cheap_hexagonal_column`] (13 vertices) with merged vertices and useful only for
 unlit games
 - [`MeshInfo::partial_hexagonal_column`] (31 vertices) without the bottom face
 - [`MeshInfo::hexagonal_column`] (38 vertices) with the bottom face

<!-- cargo-sync-readme end -->

> See the [examples](examples) for bevy usage

 ## Example

 ![example](docs/hex_grid.png?)

 > `cargo run --example hex_grid`

 ![example](docs/3d_columns.png?)

 > `cargo run --example 3d_columns`
