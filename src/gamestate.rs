use crate::prelude::*;

#[derive(Clone, Copy, Default, Debug)]
pub struct DeltaTime(pub f64);

pub struct GameState {
    runner: Runner,
}

impl State for GameState {
    fn new(world: &mut hecs::World) -> Self
    where
        Self: Sized,
    {
        load_map_system(world);
        spawn_player(world, DVec2::new(160.0, 50.0));
        spawn_bird(world);

        let mut hud = Hud::new();

        hud.add_element(
            "healthbar",
            Box::new(TextureElement {
                texture: Texture::from_path(
                    "./assets/sprites/final/hud/healthbar.png",
                    &TextureSettings::new().filter(Filter::Nearest),
                )
                .unwrap(),
                transform: Transform::new().set_translation(DVec2::new(10.0, 10.0)),
                children: vec![Box::new(RectangleElement::new(
                    Transform::new().set_translation(DVec2::new(13.0, 15.0)).set_scale(DVec2::new(29.0, 3.0)),
                    [0.952, 0.329, 0.329, 1.0],
                ))],
            }),
        );

        storage::store(hud);
        storage::store(Gravity(DVec2::new(0.0, 8.0)));
        storage::store::<Option<ButtonArgs>>(None);

        let runner = RunnerBuilder::new()
            // Physics
            .with(physics_system_bundle)
            // General systems
            .with(health_system)
            .with(update_animations)
            // Player systems
            .with(player_system_bundle)
            // Enemy systems
            .with(bird_state_system)
            .with(bird_ai_system)
            .build();

        Self { runner }
    }

    fn update(&mut self, _args: UpdateArgs, world: &mut hecs::World) {
        self.runner.run(world);
    }

    fn render(&mut self, c: Context, gl: &mut GlGraphics, _args: RenderArgs, world: &mut hecs::World) {
        // Temp solution
        render_map_system(world, c, gl);
        render_animations_system(world, c, gl);
        #[cfg(debug_assertions)]
        {
            render_debug_boxes_system(world, c, gl);
            render_bird_debug(world, c, gl);
        }

        storage::get::<Hud>().render_elements(c, gl);
    }

    fn input(&mut self, args: ButtonArgs, _world: &mut hecs::World) {
        storage::store(Some(args))
    }
}
