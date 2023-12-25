pub mod input;
pub mod unitys;
pub mod word;
use bevy::prelude::*;
use bevy_editor_pls::prelude::*;
use bevy_rapier2d::prelude::*;
use input::*;
use unitys::*;
use word::*;
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.))
        .add_plugins(EditorPlugin::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Update, bevy::window::close_on_esc)
        .add_plugins((WordPlugin, UnitysPlugin, InputPluginM))
        .run();
}
