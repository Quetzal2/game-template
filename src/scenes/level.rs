use ggez;
use ggez::graphics;
use ggez_goodies::scene;
use log::*;
//use specs::{self, Join};
use legion::prelude as legion_p;
use warmy;

use crate::components as c;
use crate::input;
use crate::resources;
use crate::scenes;
//use crate::systems::*;
use crate::world::World;
//use crate::stages;
use legion::prelude::IntoQuery;

pub struct LevelScene {
    done: bool,
    kiwi: warmy::Res<resources::Image>,
    dispatcher: legion_p::Schedule//specs::Dispatcher<'static, 'static>,
}

impl LevelScene {
    pub fn new(ctx: &mut ggez::Context, world: &mut World) -> Self {
        let done = false;
        let kiwi = world
            .resources
            .get::<resources::Image>(&resources::Key::from_path("/images/kiwi.png"), ctx)
            .unwrap();

        let mut dispatcher = Self::register_systems();
        //dispatcher.setup(&mut world.specs_world.res);

        LevelScene {
            done,
            kiwi,
            dispatcher,
        }
    }

    fn register_systems() -> legion_p::Schedule { //specs::Dispatcher<'static, 'static> {
        let update_positions = legion_p::SystemBuilder::new("update_positions")
            .with_query(<(legion_p::Write<c::Position>, legion_p::Read<c::Motion>)>::query())
            .build(|_, mut world, _, query| {
                for (mut pos, motion) in query.iter(&mut world) {
                    pos.0 += motion.velocity;
                }
            });
        let mut system_dispatcher = legion_p::Schedule::builder()
            .add_system(update_positions)
            .build();
        /*specs::DispatcherBuilder::new()
            .with(MovementSystem, "sys_movement", &[])
            .build()*/
        system_dispatcher
    }
}

impl scene::Scene<World, input::Event> for LevelScene {
    fn update(&mut self, gameworld: &mut World, _ctx: &mut ggez::Context) -> scenes::Switch {
        self.dispatcher.execute(&mut gameworld.specs_world);
        if self.done {
            scene::SceneSwitch::Pop
        } else {
            scene::SceneSwitch::None
        }
    }

    fn draw(&mut self, gameworld: &mut World, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        // update positions
        let mut query = <(legion_p::Read<c::Position>)>::query();
        for (mut p) in query.iter(&mut gameworld.specs_world) {
            graphics::draw(
                ctx,
                &(self.kiwi.borrow().0),
                graphics::DrawParam::default().dest(p.0),
            )?;
        }
        /*
        let pos = gameworld.specs_world.read_storage::<c::Position>();
        for p in pos.join() {
            graphics::draw(
                ctx,
                &(self.kiwi.borrow().0),
                graphics::DrawParam::default().dest(p.0),
            )?;
        }*/
        Ok(())
    }

    fn name(&self) -> &str {
        "LevelScene"
    }

    fn input(&mut self, gameworld: &mut World, ev: input::Event, _started: bool) {
        debug!("Input: {:?}", ev);
        if gameworld.input.get_button_pressed(input::Button::Menu) {
            self.done = true;
        }
    }
}
