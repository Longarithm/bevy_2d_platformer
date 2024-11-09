use bevy::{
    math::bounding::{Aabb2d, IntersectsVolume},
    prelude::*,
};

use crate::GameState;

use super::{Ground, Player, Velocity};

pub fn player_plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            control_player,
            velocity_update,
            transform_update,
            // Uncommend to following line after you've done the exercises 6.4 to have srpites for the ground
            // gravity
        )
            .run_if(in_state(GameState::Game)),
    );
}
fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Query<(&mut Sprite, &mut Velocity), With<Player>>,
    mut steps: Local<u32>,
) {
    let (mut sprite, mut velocity) = player.single_mut();
    velocity.target = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.target = -10.0;
        *steps += 1;
        sprite.flip_x = true;
        // player_transform.translation.x -= 5.0;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.target = 10.0;
        *steps += 1;
        sprite.flip_x = false;
        // player_transform.translation.x += 5.0;
    }
    if *steps % 10 == 1 {
        *steps %= 10;
        if sprite.texture_atlas.as_ref().unwrap().index == 0 {
            sprite.texture_atlas.as_mut().unwrap().index = 1 * 7;
        } else {
            sprite.texture_atlas.as_mut().unwrap().index = 0 * 7;
        }
    }
}

fn velocity_update(mut player: Query<&mut Velocity, With<Player>>) {
    let mut velocity = player.single_mut();
    if velocity.target != velocity.current {
        velocity.current += (velocity.target - velocity.current) / 2.0;
    }
}

fn transform_update(mut player: Query<(&mut Transform, &Velocity), With<Player>>) {
    let (mut transform, velocity) = player.single_mut();
    if velocity.current != 0.0 {
        transform.translation.x += velocity.current;
    }
}

fn gravity(
    mut player: Query<&mut Transform, With<Player>>,
    ground: Query<&Transform, (Without<Player>, With<Ground>)>,
) {
    let mut is_on_ground = false;
    let mut player_transform = player.single_mut();

    let player_aabb = Aabb2d::new(
        Vec2::new(
            player_transform.translation.x,
            player_transform.translation.y,
        ),
        Vec2::new(
            128.0 * player_transform.scale.x,
            256.0 * player_transform.scale.y,
        ) / 2.0,
    );

    for ground_transform in &ground {
        let ground_aabb = Aabb2d::new(
            Vec2::new(
                ground_transform.translation.x,
                ground_transform.translation.y,
            ),
            Vec2::new(
                128.0 * ground_transform.scale.x,
                128.0 * ground_transform.scale.y,
            ) / 2.0,
        );

        if ground_aabb.intersects(&player_aabb) {
            is_on_ground = true;
        }
    }
    if !is_on_ground {
        player_transform.translation.y -= 10.0;
    }
}
