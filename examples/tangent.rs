use bevy::{prelude::*, reflect::TypeUuid, render::render_resource::*};

use bevy_cube_sphere::CubeSphere;

#[derive(Component)]
struct Movable;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "dc8752fb-7836-4f2e-8ef8-515dcfd89248"]
struct TangentDebugMaterial {}

impl Material for TangentDebugMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/tangent_debug.wgsl".into()
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<TangentDebugMaterial>::default())
        .add_startup_system(setup)
        .add_system(object_rotate)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<TangentDebugMaterial>>,
) {
    let mesh: Mesh = CubeSphere::default().into();

    commands.spawn(MaterialMeshBundle::<TangentDebugMaterial> {
        mesh: meshes.add(mesh),
        material: materials.add(TangentDebugMaterial {}),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    let camera_origin = commands.spawn((TransformBundle::default(), Movable)).id();

    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();

    commands.entity(camera_origin).add_child(camera);
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
