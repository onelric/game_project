use crate::prelude::*;

static mut DELTATIME: Option<f64> = None;

pub struct EventHandler {
    window: Window,
    states: StateHandler,
    world: hecs::World,
}

impl EventHandler {
    pub fn new<T>() -> Self
    where
        T: State + 'static,
    {
        let window = Window::new();

        let mut world = hecs::World::default();

        let states = StateHandler::new::<T>(GlGraphics::new(window.get_opengl()), &mut world);

        Self { window, states, world }
    }

    pub fn run(&mut self) {
        let mut events = Events::new(EventSettings::new().max_fps(500));
        while let Some(e) = events.next(self.window.get_glutin_window()) {
            if let Some(args) = e.render_args() {
                self.states.render(args, &mut self.world);
            }

            if let Some(args) = e.update_args() {
                self.states.update(args, &mut self.world);
                unsafe {
                    if DELTATIME.is_none() {
                        DELTATIME = Some(args.dt);
                    }
                }
            }

            if let Some(args) = e.button_args() {
                self.states.input(args, &mut self.world);
            }
        }
    }
}

pub fn get_delta_time() -> f64 {
    unsafe { DELTATIME.unwrap_or_default() }
}
