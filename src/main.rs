use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, (setup_graphics, setup_physics, setup_character))
        .add_systems(Update, (print_ball_altitude, player_movement))
        .run();
}

fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}

fn setup_physics(mut commands: Commands, mut rapier_config: ResMut<RapierConfiguration>) {
    rapier_config.gravity = Vec2::new(0., -600.);
    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(75.0))
        .insert(Restitution::coefficient(1.))
        .insert(TransformBundle::from(Transform::from_xyz(50.0, -75.0, 50.0)));
}

#[derive(Component)]
struct Player(f32);

fn setup_character(mut commands: Commands) {
    let sprite_size = 100.0;
    commands.spawn((
            RigidBody::Dynamic,
            Velocity::zero(),
            Collider::ball(sprite_size / 2.0),
            Restitution::coefficient(1.),
            Player(100.0),
            TransformBundle::from(Transform::from_xyz(200., -100., 90.))
    ));
}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        println!("Ball altitude: x:{}, y:{}", transform.translation.x, transform.translation.y);
    }
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_info: Query<(&Player, &mut Velocity)>,
) {
    for (player, mut rb_vels) in &mut player_info {
        let left = keyboard_input.any_pressed([KeyCode::A, KeyCode::Left]);
        let right = keyboard_input.any_pressed([KeyCode::D, KeyCode::Right]);

        let x_axis = -(left as i8) + right as i8;

        let move_delta = x_axis as f32;
        // Update the velocity on the rigid_body_component,
        // the bevy_rapier plugin will update the Sprite transform.
        rb_vels.linvel.x = move_delta * player.0;
    }
}
