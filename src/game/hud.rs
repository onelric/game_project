use std::{any::Any, collections::HashMap};

use crate::prelude::{Animation, Color, Context, DVec2, GlGraphics, Texture, Transform, Transformed};

pub trait HudElement: Any + Send + Sync {
    fn render(&self, c: Context, gl: &mut GlGraphics);

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Texture element
pub struct TextureElement {
    pub texture: Texture,
    pub transform: Transform,
    pub children: Vec<Box<dyn HudElement>>,
}

impl HudElement for TextureElement {
    fn render(&self, c: Context, gl: &mut GlGraphics) {
        use graphics::image;
        for child in self.children.iter() {
            child.render(c, gl)
        }

        image(
            &self.texture,
            c.transform.trans(self.transform.translation.x, self.transform.translation.y),
            gl,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// `AnimatedElement` is used for animated sprites in the hud
pub struct AnimatedElement {
    pub animation: Animation,
    pub transform: Transform,
}

impl HudElement for AnimatedElement {
    fn render(&self, c: Context, gl: &mut GlGraphics) {
        self.animation.render(c, gl, &self.transform)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

/// Rectangular transformable hud element
#[derive(Debug)]
pub struct RectangleElement {
    transform: Transform,
    pub render_transform: Transform,
    pub color: Color,
}

impl RectangleElement {
    pub fn new(transform: Transform, color: Color) -> Self {
        Self {
            transform,
            render_transform: transform.clone(),
            color,
        }
    }
}

impl HudElement for RectangleElement {
    fn render(&self, c: Context, gl: &mut GlGraphics) {
        use graphics::Rectangle;

        Rectangle::new(self.color).draw(
            [
                self.render_transform.translation.x,
                self.render_transform.translation.y,
                self.render_transform.scale.x,
                self.render_transform.scale.y,
            ],
            &c.draw_state,
            c.transform,
            gl,
        )
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl RectangleElement {
    pub fn get_width(&self) -> f64 {
        self.transform.scale.x
    }

    pub fn get_height(&self) -> f64 {
        self.transform.scale.x
    }

    pub fn set_width(&mut self, width: f64) {
        self.render_transform.scale = DVec2::new(width, self.render_transform.scale.y)
    }

    pub fn set_height(&mut self, height: f64) {
        self.render_transform.scale = DVec2::new(self.render_transform.scale.x, height)
    }
}

/// Renders textures/rectangles/text above the game scene
pub struct Hud {
    elements: HashMap<String, Box<dyn HudElement>>,
}

impl Hud {
    pub fn new() -> Self {
        Self { elements: HashMap::new() }
    }

    pub fn add_element<T>(&mut self, element_id: &str, element: Box<T>)
    where
        T: HudElement + 'static,
    {
        self.elements.insert(element_id.to_owned(), element);
    }

    pub fn get_element<T>(&self, element_id: &str) -> Result<&T, String>
    where
        T: Any + HudElement + 'static,
    {
        for (k, v) in self.elements.iter() {
            let any = v.as_any();
            if k == element_id {
                return Ok(any.downcast_ref::<T>().unwrap());
            }
        }
        Err(format!("Could not find element: {}, in element map.", element_id))
    }

    pub fn get_element_mut<T>(&mut self, element_id: &str) -> Result<&mut T, String>
    where
        T: Any + HudElement + 'static,
    {
        for (k, v) in self.elements.iter_mut() {
            let any = v.as_any_mut();
            if k == element_id {
                return Ok(any.downcast_mut::<T>().unwrap());
            }
        }
        Err(format!("Could not find element: {}, in element map.", element_id))
    }

    pub fn clear_elements(&mut self) {
        self.elements.clear()
    }

    pub fn render_elements(&self, c: Context, gl: &mut GlGraphics) {
        for (_, element) in &self.elements {
            element.render(c, gl)
        }
    }
}
