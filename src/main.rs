pub mod basic;
use basic::*;
use bevy::{prelude::*, transform::commands};
use bevy_egui::{egui,EguiContext,EguiPlugin};

fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EguiPlugin)
    .insert_resource(LifeTimer(Timer::from_seconds(0.2, TimerMode::Repeating)))
    .init_resource::<GameState>()
    .init_resource::<UiState>()
    .add_systems(Startup, setup)
    .run();
}

/**
 *  游戏初始化
 */
fn setup(mut cmd:Commands,game_state:Res<GameState>,mut windows: Query<&mut Window>,){
    let mut window=windows.single_mut();
    window.resolution.set(WINDOW_INIT_WIDTH, WINDOW_INIT_HEIGHT);
    cmd.spawn(Camera2dBundle::default());
}