mod back_button;
mod background;
mod level;
mod loading;
mod piggies;
mod piggies_ui;
mod sling;
mod splash_screen;
mod bomb;

use avian2d::prelude::*;
use bevy::{prelude::*, window::WindowResolution};

use crate::loading::LoadingPlugin;

#[derive(Component)]
pub struct MainCamera;

/// These are the different Game State we support.
/// States allow us to trigger life-cycle events when entering,
/// leaving or Updating per frame in a certain state.
/// Furthermore using `scoped_entities` we can spawn entites
/// that get automatically cleaned up when a state is exited
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
#[states(scoped_entities)]
enum GameState {
    #[default]
    Loading,
    SplashScreen,
    Playing,
}

/// The entry function of our game
fn main() {
    let mut app = App::new();

    // We setup the Bevy Default Plugins for the most part this is just chore
    app.add_plugins(
        DefaultPlugins
            .set(AssetPlugin {
                // needed so the game works on the web
                meta_check: bevy::asset::AssetMetaCheck::Never,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Angry Bevys".to_string(),
                    canvas: Some("#bevy".to_owned()),
                    prevent_default_event_handling: true,
                    fit_canvas_to_parent: true,
                    resolution: WindowResolution::new(768.0, 640.0),
                    ..default()
                }),
                ..default()
            }),
    );

    // register physics plugins
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsDebugPlugin::default());

    app.init_state::<GameState>();

    // we configure a custom non-real gravity for or physics to make it feel more gamy
    app.insert_resource(Gravity(Vec2::NEG_Y * 9.81 * 50.0));

    // here we register all our custom plugins
    app.add_plugins((
        LoadingPlugin,
        splash_screen::plugin,
        level::plugin,
        sling::plugin,
        background::plugin,
        back_button::plugin,
        piggies::plugin,
        piggies_ui::plugin,
        bomb::plugin,
    ));

    // this runs the game
    app.run();
}
