use bevy::prelude::*;

use crate::{GameAssets, GameState};

mod player;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(player::player_plugin)
        .add_systems(OnEnter(GameState::Game), display_level);
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Ground;

#[derive(Component, Default)]
struct Velocity {
    current: f32,
    target: f32,
}

fn display_level(mut commands: Commands, assets: Res<GameAssets>) {
    for i in -3..4 {
        commands.spawn((
            // Sprite::from_color(Color::linear_rgb(0.0, 1.0, 0.0), Vec2::new(1000.0, 80.0)),
            Sprite::from_atlas_image(
                assets.ground_image.clone(),
                TextureAtlas {
                    layout: assets.ground_layout.clone(),
                    index: 1 * 7 + 0,
                },
            ),
            Transform::from_xyz(i as f32 * 128.0, -190.0, 0.0),
            Ground,
            StateScoped(GameState::Game),
        ));
    }

    commands.spawn((
        Sprite::from_atlas_image(
            assets.player_image.clone(),
            TextureAtlas {
                layout: assets.player_layout.clone(),
                index: 0,
            },
        ),
        StateScoped(GameState::Game),
        Player,
    ));

    commands.spawn(Velocity {
        current: 0.0,
        target: 0.0,
    });
}
