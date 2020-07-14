use super::{map, player, State, TileType, Vector2};

use rltk::{Rltk, VirtualKeyCode};
use specs::prelude::*;
use specs_derive::*;
use std::cmp::{max, min};

#[derive(Component, Debug)]
pub struct Player;

pub fn move_to(dx: i32, dy: i32, world: &mut World) {
    let mut positions = world.write_storage::<Vector2>();
    let mut players = world.write_storage::<Player>();
    let map = world.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_idx = map::flatten(pos.x + dx, pos.y + dy);

        if map[dest_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + dx));
            pos.y = min(49, max(0, pos.y + dy));
        }
    }
}

pub fn input(game_state: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => player::move_to(-1, 0, &mut game_state.world),
            VirtualKeyCode::Right => player::move_to(1, 0, &mut game_state.world),
            VirtualKeyCode::Up => player::move_to(0, -1, &mut game_state.world),
            VirtualKeyCode::Down => player::move_to(0, 1, &mut game_state.world),
            _ => {}
        },
    }
}
