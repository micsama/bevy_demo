use bevy::{
    audio::{PlaybackMode, Volume, VolumeLevel},
    prelude::*,
};
use bevy_rapier2d::{parry::transformation::utils::transform, prelude::*};

pub struct WordPlugin;
impl Plugin for WordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_physics, load_assets))
            .insert_resource(WordAssets::default())
            .add_systems(Startup, setup_word.after(load_assets));
    }
}

#[derive(Resource, Default)]
struct WordAssets {
    textures: Handle<Image>,
    bgm: Handle<AudioSource>,
}

//加载文件夹中的资产
fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut res: ResMut<WordAssets>,
) {
    res.bgm = asset_server.load("audio/bgm1.wav");
    res.textures = asset_server.load("img/back_1.png");
}

fn setup_word(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut Rcfg: ResMut<RapierConfiguration>,
    res: Res<WordAssets>,
) {
    // 添加摄像头以及设置重力
    commands.spawn(Camera2dBundle::default());
    Rcfg.gravity = Vec2::new(10., -100.);
    // 设置背景图片以及bgm
    commands.spawn(AudioBundle {
        source: res.bgm.clone(),
        settings: PlaybackSettings {
            mode: PlaybackMode::Loop,
            ..default()
        },
    });

    //背景图片
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 900., y: 900. }),
            ..default()
        },
        texture: res.textures.clone(),
        ..default()
    });

    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(20.))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 200.0, 0.0)));

    commands.spawn(Collider::polyline(
        get_circle_points(3., -25., 400., 20),
        None,
    ));
    // commands.spawn(Collider::ball(400.))
    //     // .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));
    //     .insert(TransformBundle::from(Transform::from_xyz(3.0, -23.0, 0.0)));
}

fn setup_physics(mut commands: Commands, mut windows: Query<&mut Window>) {
    windows.single_mut().resolution.set(900., 900.);
}

fn get_circle_points(m: f32, n: f32, r: f32, n_points: usize) -> Vec<Vec2> {
    let mut points: Vec<Vec2> = (0..n_points)
        .map(|i| {
            // 将角度转换为弧度
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (n_points as f32);
            // 计算圆周上的点
            Vec2 {
                x: m + r * angle.cos(),
                y: n + r * angle.sin(),
            }
        })
        .collect();

    // 为了闭合圆，将第一个点再添加到末尾
    if let Some(first_point) = points.first().cloned() {
        points.push(first_point);
    }
    points
}
