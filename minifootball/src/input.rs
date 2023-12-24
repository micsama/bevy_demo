use crate::unitys::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use minifootball::*;

pub struct InputPluginM;
impl Plugin for InputPluginM {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse);
    }
}

fn mouse(
    mut commands: Commands,
    server: Res<AssetServer>,
    assets: Res<UnitAssets>,
    mouse: Res<Input<MouseButton>>,
    worldcoords: Res<MyWorldCoords>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        commands
            .spawn(SpriteBundle {
                texture: server.load(assets.assets_2[0].clone()),
                transform: Transform {
                    translation: Vec3::new(worldcoords.0.x, worldcoords.0.y, 1.),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 40., y: 40. }),
                    ..default()
                },
                ..default()
            })
            .insert(Collider::ball(20.))
            .insert(RigidBody::Dynamic)
            .insert(Friction::coefficient(1.))
            .insert(Restitution::coefficient(1.))
            
            ;
    };
}
