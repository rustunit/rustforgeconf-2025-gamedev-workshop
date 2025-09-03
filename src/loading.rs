/// Our LoadingPlugin uses `bevy_asset_loader` to preload all assets
/// we are using in one go and not move on before they are done loading.
/// This helps with the async nature of asset loading otherwise.
/// Once all assets are loaded we move over to `GameState::SplashScreen`
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{GameState, MainCamera};

#[derive(AssetCollection, Resource)]
pub struct GameAssets {
    #[asset(path = "bevy_bird_dark.png")]
    pub bevy: Handle<Image>,

    #[asset(path = "alienPink_square.png")]
    pub piggy: Handle<Image>,

    #[asset(path = "bomb.png")]
    pub bomb: Handle<Image>,

    #[asset(path = "elementWood022.png")]
    pub kiste: Handle<Image>,

    #[asset(path = "elementWood008.png")]
    pub roof: Handle<Image>,

    #[asset(path = "background/colored_grass.png")]
    pub background: Handle<Image>,
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnExit(GameState::Loading), on_exit)
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::SplashScreen)
                    .load_collection::<GameAssets>(),
            );
    }
}

/// here we spawn our camera once
fn on_exit(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
        Msaa::Off,
        Transform::from_scale(Vec3::splat(1.6)),
    ));
}
