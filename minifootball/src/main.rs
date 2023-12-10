use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_editor_pls::prelude::*;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EditorPlugin::default())
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_systems(Startup, (setup_graphics,setup_physics,modify_body_gravity_scale))
    .add_systems(Update, print_ball_altitude)
    .init_resource::<RapierConfiguration>()
    .run()
}


fn setup_graphics(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands.spawn(Camera2dBundle::default());
}
fn modify_body_gravity_scale(mut Rapcfg: ResMut<RapierConfiguration>) {
    Rapcfg.gravity=Vec2::new(-10.,-180.);
}



fn setup_physics(mut commands: Commands) {

    /* Create the ground. */
    commands
        .spawn(Collider::cuboid(500.0, 50.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -200.0, 0.0)));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(50.0))
        .insert(Restitution::coefficient(1.4))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));

}

fn print_ball_altitude(positions: Query<&Transform, With<RigidBody>>) {
    for transform in positions.iter() {
        // println!("Ball altitude: {}", transform.translation.y);
    }
}