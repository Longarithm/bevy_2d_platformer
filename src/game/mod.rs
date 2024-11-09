use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};

use crate::{
    level_loader::{Level, LoadedLevel, Tile},
    GameAssets, GameState,
};

mod player;

const SCALE: f32 = 0.5;

pub fn game_plugin(app: &mut App) {
    app.add_plugins(player::player_plugin)
        .add_systems(OnEnter(GameState::Game), display_level)
        .add_systems(
            Update,
            animate_level.run_if(on_timer(Duration::from_secs_f32(0.25))),
        );
}

#[derive(Component)]
#[require(IsOnGround, Velocity, AgainstWall, PoweredUp)]
struct Player;

#[derive(Component, Default)]
struct IsOnGround(f32);

#[derive(Component, Default)]
struct AgainstWall(bool, bool);

#[derive(Component, Default)]
struct Velocity {
    current: f32,
    target: f32,
    jumping: f32,
}

#[derive(Component, Default)]
struct PoweredUp(bool);

#[derive(Component)]
struct Ground;

#[derive(Component)]
struct Flag;

#[derive(Event)]
struct ReachedFlag;

#[derive(Component)]
struct PowerUp;

#[derive(Event)]
struct ReachedPowerUp;

fn ground_tile_index(line: &[Tile], i: usize) -> usize {
    match (
        i == 0 || !matches!(line.get(i - 1).unwrap_or(&Tile::Empty), Tile::Ground),
        !matches!(line.get(i + 1).unwrap_or(&Tile::Empty), Tile::Ground),
    ) {
        (true, true) => 8,
        (true, false) => 14,
        (false, true) => 0,
        (false, false) => 7,
    }
}

fn display_tile(
    commands: &mut Commands,
    tile: &Tile,
    i: usize,
    x: f32,
    y: f32,
    line: &[Tile],
    assets: &GameAssets,
) {
    match tile {
        Tile::Ground => {
            let index = ground_tile_index(line, i);
            commands.spawn((
                Sprite::from_atlas_image(
                    assets.ground_image.clone(),
                    TextureAtlas {
                        layout: assets.ground_layout.clone(),
                        index,
                    },
                ),
                Transform::from_xyz(x, y, 0.0).with_scale(Vec3::splat(SCALE)),
                Ground,
                StateScoped(GameState::Game),
            ));
        }
        Tile::Spawn => {
            commands.spawn((
                Sprite::from_atlas_image(
                    assets.player_image.clone(),
                    TextureAtlas {
                        layout: assets.player_layout.clone(),
                        index: 0,
                    },
                ),
                Transform::from_xyz(x, y + 256.0 / 4.0 * SCALE, 2.0).with_scale(Vec3::splat(SCALE)),
                StateScoped(GameState::Game),
                Player,
            ));
        }
        Tile::Flag => {
            commands
                .spawn((
                    Sprite::from_atlas_image(
                        assets.items_image.clone(),
                        TextureAtlas {
                            layout: assets.items_layout.clone(),
                            index: 6,
                        },
                    ),
                    Transform::from_xyz(x, y, 1.0).with_scale(Vec3::splat(SCALE)),
                    StateScoped(GameState::Game),
                    Flag,
                ))
                .observe(reached_flag);
        }
        Tile::PowerUp => {
            commands
                .spawn((
                    Sprite::from_atlas_image(
                        assets.tiles_image.clone(),
                        TextureAtlas {
                            layout: assets.tiles_layout.clone(),
                            index: 7 * 5 + 2,
                        },
                    ),
                    Transform::from_xyz(x, y, 1.0).with_scale(Vec3::splat(SCALE)),
                    StateScoped(GameState::Game),
                    PowerUp,
                ))
                .observe(reached_power_up);
        }
        Tile::Empty => {}
    }
}

fn display_level(
    mut commands: Commands,
    assets: Res<GameAssets>,
    level: Res<LoadedLevel>,
    levels: Res<Assets<Level>>,
) {
    let level = levels.get(&level.level).unwrap();

    for (j, line) in level.tiles.iter().enumerate() {
        for (i, tile) in line.iter().enumerate() {
            let (x, y) = (
                (i as f32 - 9.0) * 128.0 * SCALE,
                -(j as f32 - 5.0) * 128.0 * SCALE,
            );
            display_tile(&mut commands, tile, i, x, y, line, &assets);
        }
    }
}

fn animate_level(mut flags: Query<&mut Sprite, With<Flag>>) {
    for mut flag in &mut flags {
        let atlas = flag.texture_atlas.as_mut().unwrap();
        if atlas.index == 6 {
            atlas.index = 12;
        } else {
            atlas.index = 6;
        }
    }
}

fn reached_flag(_trigger: Trigger<ReachedFlag>, mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::Menu);
}

fn reached_power_up(
    _trigger: Trigger<ReachedPowerUp>,
    mut players: Query<&mut Sprite, With<Player>>,
) {
    // for mut player in &mut players {
    //     player.texture_atlas.as_mut().unwrap().index = 20;
    // }
}
