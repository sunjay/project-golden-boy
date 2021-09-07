use std::f32::consts::PI;

use bevy::{
    core::FixedTimestep,
    prelude::*,
    render::pass::ClearColor,
    ecs::system::EntityCommands,
};
use bevy_rapier2d::{prelude::*, na};

const FPS: f32 = 60.0;
const TIME_STEP: f32 = 1.0 / FPS;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .insert_resource(ClearColor(Color::rgb(0.6, 0.6, 0.6)))
        .add_startup_system(setup.system())
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(keyboard_control.system())
        )
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .run();
}

struct KeyboardControlled;

fn setup(
    mut commands: Commands,
    mut physics_config: ResMut<RapierConfiguration>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    // Top-down game, so no gravity
    physics_config.gravity.x = 0.0;
    physics_config.gravity.y = 0.0;

    // Cameras
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());

    // Walls
    let wall_material = materials.add(Color::rgb(0.5, 0.5, 1.0).into());
    let wall_thickness = 10.0;
    let bounds = Vec2::new(800.0, 600.0);

    // Left Wall
    spawn_rectangle(
        commands.spawn(),
        wall_material.clone(),
        transform2d(-bounds.x / 2.0, 0.0, 0.0, 0.0),
        Vec2::new(wall_thickness, bounds.y),
    );
    // Right Wall
    spawn_rectangle(
        commands.spawn(),
        wall_material.clone(),
        transform2d(bounds.x / 2.0, 0.0, 0.0, 0.0),
        Vec2::new(wall_thickness, bounds.y),
    );
    // Top Wall
    spawn_rectangle(
        commands.spawn(),
        wall_material.clone(),
        transform2d(0.0, -bounds.y / 2.0, 0.0, 0.0),
        Vec2::new(bounds.x, wall_thickness),
    );
    // Bottom Wall
    spawn_rectangle(
        commands.spawn(),
        wall_material.clone(),
        transform2d(0.0, bounds.y / 2.0, 0.0, 0.0),
        Vec2::new(bounds.x, wall_thickness),
    );

    // Additional Walls
    spawn_rectangle(
        commands.spawn(),
        wall_material.clone(),
        transform2d(-bounds.x / 2.0 + 200.0, bounds.y / 2.0 - 300.0, 0.0, PI / 3.0),
        Vec2::new(bounds.x / 4.0, wall_thickness * 4.0),
    );
    spawn_rectangle(
        commands.spawn(),
        wall_material.clone(),
        transform2d(bounds.x / 2.0 - 200.0, bounds.y / 2.0 - 300.0, 0.0, -PI / 2.5),
        Vec2::new(bounds.x / 2.0, wall_thickness * 2.0),
    );

    // Player
    spawn_rigid_body_rectangle(
        commands.spawn(),
        materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        transform2d(0.0, 0.0, 1.0, 0.0),
        Vec2::new(30.0, 30.0),
    ).insert(KeyboardControlled);
}

fn transform2d(x: f32, y: f32, z: f32, radians: f32) -> Transform {
    Transform {
        translation: Vec3::new(x, y, z),
        rotation: Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), radians),
        ..Transform::default()
    }
}

fn spawn_rigid_body_rectangle<'a, 'b>(
    entity: EntityCommands<'a, 'b>,
    material: Handle<ColorMaterial>,
    transform: Transform,
    size: Vec2,
) -> EntityCommands<'a, 'b> {
    let mut entity = spawn_rectangle(entity, material, transform, size);
    entity.insert_bundle(RigidBodyBundle {
        position: [transform.translation.x, transform.translation.y].into(),
        mass_properties: RigidBodyMassPropsFlags::ROTATION_LOCKED.into(),
        ..Default::default()
    });
    entity
}

fn spawn_rectangle<'a, 'b>(
    mut entity: EntityCommands<'a, 'b>,
    material: Handle<ColorMaterial>,
    transform: Transform,
    size: Vec2,
) -> EntityCommands<'a, 'b> {
    entity.insert_bundle(SpriteBundle {
        material,
        transform,
        sprite: Sprite::new(size),
        ..Default::default()
    }).insert_bundle(ColliderBundle {
        shape: ColliderShape::cuboid(size.x / 2.0, size.y / 2.0),
        position: na::Isometry2::new(
            [transform.translation.x, transform.translation.y].into(),
            transform.rotation.to_axis_angle().1,
        )
        .into(),
        ..Default::default()
    }).insert(ColliderPositionSync::Discrete);
    entity
}

fn keyboard_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&KeyboardControlled, &mut RigidBodyVelocity)>,
) {
    if let Ok((_, mut vel)) = query.single_mut() {
        let mut direction = Vector::zeros();
        if keyboard_input.pressed(KeyCode::Left) {
            direction += -1.0f32 * Vector::x();
        }

        if keyboard_input.pressed(KeyCode::Right) {
            direction += 1.0f32 * Vector::x();
        }

        if keyboard_input.pressed(KeyCode::Down) {
            direction += -1.0f32 * Vector::y();
        }

        if keyboard_input.pressed(KeyCode::Up) {
            direction += 1.0f32 * Vector::y();
        }

        const SPEED: f32 = 200.0;
        vel.linvel = SPEED * direction;
    }
}
