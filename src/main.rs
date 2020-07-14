mod components;
mod map;
mod player;
mod rect;

use rltk::{GameState, Rltk, RltkBuilder, RGB};
use specs::prelude::*;

use map::TileType;

use player::Player;

use components::Renderable;
use components::Vector2;

pub struct State {
    pub world: World,
}

impl State {
    fn new() -> Self {
        State {
            world: World::new(),
        }
    }

    fn run_systems(&mut self) {
        // let mut lw = LeftWalker;
        // lw.run_now(&self.world);
        // self.world.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player::input(self, ctx);

        self.run_systems();

        let map = self.world.fetch::<Vec<TileType>>();
        map::draw(&map, ctx);

        let positions = self.world.read_storage::<Vector2>();
        let renderables = self.world.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn main() {
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike")
        .build()
        .expect("Building Context failed!");

    let mut game_state = State::new();

    game_state.world.register::<Vector2>();
    game_state.world.register::<Renderable>();
    game_state.world.register::<Player>();

    let (rooms, map) = map::new_map_rooms_and_corridors();

    game_state.world.insert(map);
    let player_position = rooms[0].center();
    game_state
        .world
        .create_entity()
        .with(player_position)
        .with(Renderable::new(
            rltk::to_cp437('@'),
            RGB::named(rltk::YELLOW),
            RGB::named(rltk::BLACK),
        ))
        .with(Player)
        .build();

    rltk::main_loop(context, game_state);
}
