use super::{xy_idx, Player, Position, State, TileType};
use rltk::Rltk;
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.read_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            rltk::VirtualKeyCode::H
            | rltk::VirtualKeyCode::Numpad4
            | rltk::VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            rltk::VirtualKeyCode::L
            | rltk::VirtualKeyCode::Numpad6
            | rltk::VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            rltk::VirtualKeyCode::K | rltk::VirtualKeyCode::Numpad8 | rltk::VirtualKeyCode::Up => {
                try_move_player(0, -1, &mut gs.ecs)
            }
            rltk::VirtualKeyCode::J
            | rltk::VirtualKeyCode::Numpad2
            | rltk::VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),

            _ => {}
        },
    }
}
