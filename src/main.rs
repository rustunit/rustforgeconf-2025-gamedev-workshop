mod level;
mod loading;
mod sling;
mod splash_screen;

use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

use crate::loading::LoadingPlugin;

#[derive(Component)]
pub struct MainCamera;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[states(scoped_entities)]
enum GameState {
    #[default]
    Loading,
    SplashScreen,
    Playing,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Angry Bevys".to_string(),
                    // Bind to canvas included in `index.html`
                    canvas: Some("#bevy".to_owned()),
                    // Tells wasm not to override default event handling, like F5 and Ctrl+R
                    prevent_default_event_handling: false,
                    resolution: WindowResolution::new(768.0, 640.0),
                    ..default()
                }),
                ..default()
            }),
    );

    // if !app.is_plugin_added::<EguiPlugin>() {
    //     app.add_plugins(EguiPlugin::default());
    // }

    app.init_state::<GameState>();
    app.insert_resource(Gravity(Vec2::NEG_Y * 9.81 * 50.0));

    app.add_plugins((
        LoadingPlugin,
        splash_screen::plugin,
        level::plugin,
        sling::plugin,
    ));

    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsDebugPlugin::default());

    app.run();
}
