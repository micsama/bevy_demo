pub mod word;
pub mod input;
pub mod unitys;
use word::*;
use input::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_editor_pls::prelude::*;
use unitys::*;
fn main() {
    App::new()
    .add_plugins(DefaultPlugins)
    .add_plugins(EditorPlugin::default())
    .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
    .add_plugins(RapierDebugRenderPlugin::default())
    .add_systems(Update, bevy::window::close_on_esc)
    .add_plugins((WordPlugin,UnitysPlugin,InputPluginM))
    .run();
}