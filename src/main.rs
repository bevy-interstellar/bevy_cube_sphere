use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{render_resource::WgpuFeatures, settings::WgpuSettings, RenderPlugin},
};

use bevy::render::render_resource::Face;
use bevy_cube_sphere::CubeSphere;

#[derive(Component)]
struct Movable;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WgpuSettings {
            features: WgpuFeatures::POLYGON_MODE_LINE,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(WireframePlugin)
        .add_startup_system(setup)
        .add_system(object_rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut wireframe_config: ResMut<WireframeConfig>,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    wireframe_config.global = false;

    let mesh: Mesh = CubeSphere::default().into();
    let texture_handle = asset_server.load("texture.png");

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                cull_mode: Some(Face::Back),
                ..default()
            }),
            ..default()
        },
        Wireframe,
        Movable,
    ));

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn object_rotate(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        if input.pressed(KeyCode::Up) {
            transform.rotate_x(time.delta_seconds());
        }
        if input.pressed(KeyCode::Down) {
            transform.rotate_x(-time.delta_seconds());
        }
        if input.pressed(KeyCode::Left) {
            transform.rotate_y(time.delta_seconds());
        }
        if input.pressed(KeyCode::Right) {
            transform.rotate_y(-time.delta_seconds());
        }
    }
}
