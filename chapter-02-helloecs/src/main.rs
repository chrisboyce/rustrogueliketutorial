use legion::prelude::*;
use rltk::{GameState, Rltk, VirtualKeyCode, RGB};
use std::cmp::{max, min};

struct Position {
    x: i32,
    y: i32,
}

struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

struct LeftMover {}

struct Player {}

struct State {
    ecs: World,
    walker_schedule: Schedule,
    resources: Resources,
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let query = <(Read<Player>, Write<Position>)>::query();

    for (_player, mut pos) in query.iter_mut(ecs) {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {} // Nothing happened
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let query = <(Read<Position>, Read<Renderable>)>::query();
        for (pos, render) in query.iter(&self.ecs) {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

fn create_walker_system() -> Box<dyn Schedulable> {
    let system = SystemBuilder::new("left_walker")
        .with_query(<(Read<LeftMover>, Write<Position>)>::query())
        .build(|_command_buffer, world, _time, query| {
            for (_, mut pos) in query.iter_mut(world) {
                pos.x -= 1;
                if pos.x < 0 {
                    pos.x = 79;
                }
            }
        });
    system
}
fn create_resources() -> Resources {
    Resources::default()
}

impl State {
    fn run_systems(&mut self) {
        self.walker_schedule
            .execute(&mut self.ecs, &mut self.resources);
    }
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;
    let walker_system = create_walker_system();
    let schedule = Schedule::builder().add_system(walker_system).build();
    let mut gs = State {
        ecs: Universe::new().create_world(),
        walker_schedule: schedule,
        resources: create_resources(),
    };

    gs.ecs.insert(
        (),
        (0..1).map(|_| {
            (
                Position { x: 40, y: 25 },
                Renderable {
                    glyph: rltk::to_cp437('@'),
                    fg: RGB::named(rltk::YELLOW),
                    bg: RGB::named(rltk::BLACK),
                },
                Player {},
            )
        }),
    );

    gs.ecs.insert(
        (),
        (0..10).map(|i| {
            (
                Position { x: i * 7, y: 20 },
                Renderable {
                    glyph: rltk::to_cp437('☺'),
                    fg: RGB::named(rltk::RED),
                    bg: RGB::named(rltk::BLACK),
                },
                LeftMover {},
            )
        }),
    );

    rltk::main_loop(context, gs)
}
