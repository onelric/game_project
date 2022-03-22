use hecs::World;

pub type SystemFn = fn(&mut World);

pub struct RunnerBuilder {
    systems: Vec<SystemFn>,
}

impl RunnerBuilder {
    pub fn new() -> Self {
        Self { systems: vec![] }
    }

    #[must_use]
    pub fn with(mut self, system: SystemFn) -> Self {
        self.systems.push(system);
        self
    }

    pub fn build(self) -> Runner {
        Runner { systems: self.systems }
    }
}

pub struct Runner {
    systems: Vec<SystemFn>,
}

impl Runner {
    pub fn run(&mut self, world: &mut World) {
        for s in self.systems.iter() {
            s(world)
        }
    }
}
