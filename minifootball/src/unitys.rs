use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct UnitysPlugin;
impl Plugin for UnitysPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(UnitAssets::default())
            .add_systems(Startup, spawn_player_node);
    }
}

#[derive(Resource,Clone)]
pub struct UnitAssets {
    pub assets_1: Vec<(String, Vec2)>, // 使用元组来存储配对的 assets 和 points
    pub assets_2: Vec<String>,
}
impl Default for UnitAssets {
    fn default() -> Self {
        const X_OFFSET: f32 = 3.0;
        const Y_OFFSET: f32 = -10.0;
        UnitAssets {
            assets_1: vec![
                ("img/bugs/a1.png".to_string(), Vec2 { x: 0. + X_OFFSET, y: -300. + Y_OFFSET }),
                ("img/bugs/b1.png".to_string(), Vec2 { x: 251. + X_OFFSET, y: 150. + Y_OFFSET }),
                ("img/bugs/c1.png".to_string(), Vec2 { x: -256. + X_OFFSET, y: 145. + Y_OFFSET }),
            ],
            assets_2: vec![
                "img/bugs/a2.png".to_string(),
                "img/bugs/b2.png".to_string(),
                "img/bugs/c2.png".to_string(),
            ],
        }
    }
}


fn spawn_player_node(mut cmd: Commands, server: Res<AssetServer>, word_assets: Res<UnitAssets>) {
    for (asset, point) in &word_assets.assets_1 {
        cmd.spawn(SpriteBundle {
            transform: Transform {
                translation: Vec3::new(point.x, point.y, 1.),
                ..default()
            },
            sprite: Sprite {
                custom_size: Some(Vec2 { x: 35., y: 35. }),
                ..default()
            },
            texture: server.load(asset.clone()),
            ..default()
        })
        .insert(Collider::ball(15.))
        .insert(RigidBody::Dynamic);
    }
}


