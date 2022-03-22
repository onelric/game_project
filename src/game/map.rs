use crate::prelude::{Context, DVec2, GlGraphics, StaticBody, Transform, World};
use gridmapper_loader::Map;

pub fn load_map_system(world: &mut World) {
    let map = Map::load("assets/maps/map");

    for tile in &map.tiles {
        world.spawn((
            StaticBody,
            Transform::new()
                .set_translation(DVec2::new(tile.x, tile.y))
                .set_scale(DVec2::new(tile.w, tile.h)),
        ));
    }
    world.spawn((map,));
}

pub fn render_map_system(world: &mut World, c: Context, gl: &mut GlGraphics) {
    world.query::<&Map>().iter().for_each(|(_, map)| map.render(c, gl));
}
