extern crate minifootball;
use bevy::{audio::{PlaybackMode, Volume, VolumeLevel}, prelude::*};
use bevy_rapier2d::prelude::*;
use minifootball::*;

pub struct WordPlugin;
impl Plugin for WordPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_physics, load_word_assets))
            .insert_resource(WordAssets::default())
            .insert_resource(MyWorldCoords::default())
            .add_systems(Startup, setup_word.after(load_word_assets))
            .add_systems(Update, my_cursor_system);
    }
}

#[derive(Resource, Default)]
struct WordAssets {
    textures: Handle<Image>,
    bgm: Handle<AudioSource>,
}

//加载文件夹中的资产
fn load_word_assets(
    // mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut res: ResMut<WordAssets>,
) {
    res.bgm = asset_server.load("audio/bgm1.wav");
    res.textures = asset_server.load("img/back_1.png");
}

fn setup_word(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    mut rcfg: ResMut<RapierConfiguration>,
    res: Res<WordAssets>,
) {

    // 添加摄像头以及设置重力
    commands.spawn((Camera2dBundle::default(), MainCamera));
    rcfg.gravity = Vec2::new(0., -160.);
    // 设置背景图片以及bgm
    commands.spawn(AudioBundle {
        source: res.bgm.clone(),
        settings: PlaybackSettings {
            volume:Volume::Relative(VolumeLevel::new(0.8)),
            mode: PlaybackMode::Loop,
            ..default()
        },
    });


    commands.spawn(SpriteBundle {
        sprite: Sprite {
            custom_size: Some(Vec2 { x: 900., y: 900. }),
            ..default()
        },
        texture: res.textures.clone(),
        ..default()
    });


    //测试的小球
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(10.))
    //     .insert(Restitution::coefficient(1.))
    //     .insert(TransformBundle::from(Transform::from_xyz(200.0, 200.0, 0.0)));

    //场地的基础碰撞
    commands.spawn(Collider::polyline(
        get_circle_points(3., -25., 400., 20),
        None,
    ));
}

fn setup_physics( mut windows: Query<&mut Window>, mut audio:ResMut<GlobalVolume>) {
    //设置全局音量
    audio.volume=VolumeLevel::new(0.8);
    windows.single_mut().resolution.set(900., 900.);
}
