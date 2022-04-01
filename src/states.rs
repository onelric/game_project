#![allow(unused)]

use crate::prelude::{ButtonArgs, Config, Context, Event, GlGraphics, RenderArgs, UpdateArgs, World};

pub trait State {
    fn new(world: &mut World) -> Self
    where
        Self: Sized;

    fn update(&mut self, args: UpdateArgs, world: &mut World) {}

    fn render(&mut self, c: Context, gl: &mut GlGraphics, args: RenderArgs, world: &mut World) {}

    fn input(&mut self, args: ButtonArgs, world: &mut World) {}

    fn events(&mut self, event: &Event, world: &mut World) {}
}

pub struct StateHandler {
    gl: GlGraphics,
    states: Vec<Box<dyn State>>,
    state: usize,
}

impl StateHandler {
    pub fn new<T>(gl: GlGraphics, world: &mut World) -> Self
    where
        T: State + 'static,
    {
        Self {
            gl,
            states: vec![Box::new(T::new(world))],
            state: 0,
        }
    }

    pub fn update(&mut self, args: UpdateArgs, world: &mut World) {
        if !self.states.is_empty() {
            self.states[self.state].update(args, world)
        }
    }

    pub fn render(&mut self, args: RenderArgs, world: &mut World) {
        let config = crate::prelude::storage::get::<Config>();

        let mut viewport = args.viewport();
        viewport.draw_size[0] = (viewport.window_size[0] * (viewport.window_size[0] / config.window.width as f64)) as u32;
        viewport.draw_size[1] = (viewport.window_size[1] * (viewport.window_size[1] / config.window.height as f64)) as u32;

        self.gl.draw(viewport, |c, gl| {
            graphics::clear([0.06, 0.06, 0.06, 1.0], gl);
            if !self.states.is_empty() {
                self.states[self.state].render(c, gl, args, world)
            }
        });
    }

    pub fn input(&mut self, args: ButtonArgs, world: &mut World) {
        if !self.states.is_empty() {
            self.states[self.state].input(args, world)
        }
    }

    pub fn events(&mut self, event: &Event, world: &mut World) {
        if !self.states.is_empty() {
            self.states[self.state].events(event, world)
        }
    }

    pub fn push<T>(&mut self, world: &mut World)
    where
        T: State + 'static,
    {
        self.states.push(Box::new(T::new(world)));
    }

    pub fn push_and_set<T>(&mut self, world: &mut World)
    where
        T: State + 'static,
    {
        self.push::<T>(world);

        // Change state to last index/newest state added
        self.state = self.states.len() - 1;
    }

    pub fn change_state(&mut self, state: usize, world: &mut World) {
        world.clear();
        self.state = state;
    }

    pub fn states(&self) -> &Vec<Box<dyn State>> {
        &self.states
    }
}
