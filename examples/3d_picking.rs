use bevy::{
    color::palettes::css::{WHITE, YELLOW},
    prelude::*,
    render::{mesh::Indices, render_asset::RenderAssetUsages, render_resource::PrimitiveTopology},
    utils::HashMap,
    window::PrimaryWindow,
};
use hexx::{shapes, *};
use light_consts::lux;

/// World size of the hexagons (outer radius)
const HEX_SIZE: Vec2 = Vec2::splat(1.0);
/// World space height of hex columns
const COLUMN_HEIGHT: f32 = 10.0;
/// Map radius
const MAP_RADIUS: u32 = 20;

pub fn main() {
    App::new()
        .insert_resource(AmbientLight {
            brightness: lux::OFFICE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (setup_camera, setup_grid))
        .add_systems(Update, higlight_hovered)
        .run();
}

#[derive(Debug, Resource)]
struct Map {
    layout: HexLayout,
    entities: HashMap<Hex, Entity>,
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

/// 3D Orthogrpahic camera setup
fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(Camera3dBundle {
        transform,
        ..default()
    });
    let transform = Transform::from_xyz(60.0, 60.0, 00.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn(DirectionalLightBundle {
        transform,
        ..default()
    });
}

/// Hex grid setup
fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let layout = HexLayout {
        hex_size: HEX_SIZE,
        ..default()
    };
    // materials
    let default_material = materials.add(Color::Srgba(WHITE));
    let highlighted_material = materials.add(Color::Srgba(YELLOW));
    // mesh
    let mesh = hexagonal_column(&layout);
    let mesh_handle = meshes.add(mesh);

    let entities = shapes::hexagon(Hex::ZERO, MAP_RADIUS)
        .map(|hex| {
            let pos = layout.hex_to_world_pos(hex);
            let id = commands
                .spawn(PbrBundle {
                    transform: Transform::from_xyz(pos.x, -COLUMN_HEIGHT, pos.y),
                    mesh: mesh_handle.clone(),
                    material: default_material.clone_weak(),
                    ..default()
                })
                .id();
            (hex, id)
        })
        .collect();
    commands.insert_resource(Map {
        layout,
        entities,
        highlighted_material,
        default_material,
    });
}

fn higlight_hovered(
    mut commands: Commands,
    map: Res<Map>,
    mut highlighted: Local<Hex>,
    cameras: Query<(&Camera, &GlobalTransform)>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows.single();
    let (camera, cam_transform) = cameras.single();
    let Some(ray) = window
        .cursor_position()
        .and_then(|p| camera.viewport_to_world(cam_transform, p))
    else {
        return;
    };
    let Some(distance) = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d::new(Dir3::Y)) else {
        return;
    };
    let point = ray.origin + ray.direction * distance;
    let coord = map.layout.world_pos_to_hex(point.xz());
    if coord != *highlighted {
        let Some(entity) = map.entities.get(&coord).copied() else {
            return;
        };
        commands
            .entity(entity)
            .insert(map.highlighted_material.clone_weak());
        commands
            .entity(map.entities[&*highlighted])
            .insert(map.default_material.clone_weak());
        *highlighted = coord;
    }
}

/// Compute a bevy mesh from the layout
fn hexagonal_column(hex_layout: &HexLayout) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, COLUMN_HEIGHT)
        .without_bottom_face()
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
