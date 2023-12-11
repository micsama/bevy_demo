use crate::unitys::*;
use bevy::{prelude::*, window::PrimaryWindow};

pub struct InputPluginM;
impl Plugin for InputPluginM {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, mouse);
    }
}


fn mouse(
    mut commands: Commands,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    server: Res<AssetServer>,
    assets: Res<UnitAssets>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        if let Some(position) = q_windows.single().cursor_position() {
            commands.spawn(SpriteBundle {
                
                texture: server.load(assets.assets_2[0].clone()),
                transform: Transform {
                    translation: Vec3::new(position.x, position.y, 1.),
                    ..default()
                },
                sprite: Sprite {
                    custom_size: Some(Vec2 { x: 40., y: 40. }),
                    ..default()
                },
                ..default()
            });
        }
    };
}
