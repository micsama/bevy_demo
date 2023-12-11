use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct UnitysPlugin;
impl Plugin for UnitysPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WordAssets::default())
            .add_systems(Startup, spawn_player_node);
    }
}

#[derive(Resource)]
struct WordAssets {
    assets_1: Vec<String>,
    assets_2: Vec<String>,
}
impl Default for WordAssets {
    fn default() -> Self {
        WordAssets {
            assets_1: vec![
                "img/bugs/a1.png".to_string(),
                "img/bugs/b1.png".to_string(),
                "img/bugs/c1.png".to_string(),
            ],
            assets_2: vec![
                "img/bugs/a2.png".to_string(),
                "img/bugs/b2.png".to_string(),
                "img/bugs/c2.png".to_string(),
            ],
        }
    }
}

fn spawn_player_node(mut cmd: Commands, server: Res<AssetServer>, res: ResMut<WordAssets>) {
    const POINTS: [Vec2; 3] = [
        Vec2 { x: 300.0, y: 0.0 },
        Vec2 {
            x: -150.00002,
            y: 259.80762,
        },
        Vec2 {
            x: -149.99997,
            y: -259.80765,
        },
    ];

    cmd.spawn(SpriteBundle {
        transform: Transform {
            translation: Vec3::new(0., 300., 1.),
            ..default()
        },
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 35., y: 35. }),
            ..default()
        },
        texture: server.load(res.assets_1[0].clone()),
        ..default()
    })
    .insert(Collider::ball(15.));
}
