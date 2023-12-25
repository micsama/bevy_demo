use  bevy::prelude::*;
use snake::*;
use bevy::audio::{Volume, VolumeLevel, PlaybackMode};
use snake::MainCamera;

pub struct myUiPlugin;
impl Plugin for myUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_word,load_word_assets))
        .insert_resource(WordAssets::default())
        .insert_resource(MyWorldCoords::default())

        ;
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

fn setup_word(mut cmd:Commands,res:Res<WordAssets>){
    cmd.spawn((Camera2dBundle::default(),MainCamera));
    cmd.spawn(SpriteBundle::default());
    cmd.spawn(AudioBundle {
        source: res.bgm.clone(),
        settings: PlaybackSettings {
            volume:Volume::Relative(VolumeLevel::new(0.1)),
            mode: PlaybackMode::Loop,
            ..default()
        },
    });
}