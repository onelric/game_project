use glam::DVec2;

#[derive(Debug, Default, Clone, Copy)]
pub struct Transform {
    pub translation: DVec2,
    pub scale: DVec2,
}

impl Transform {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_scale(mut self, scale: DVec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn set_translation(mut self, translation: DVec2) -> Self {
        self.translation = translation;
        self
    }

    /// Gets the scale divided by two
    pub fn half_scale(&self) -> DVec2 {
        DVec2::new(self.scale.x / 2.0, self.scale.y / 2.0)
    }

    /// Gets the scale divided by two plus the translation
    pub fn center(&self) -> DVec2 {
        DVec2::new(self.translation.x + self.scale.x / 2.0, self.translation.y + self.scale.y / 2.0)
    }

    pub fn left(&self) -> f64 {
        self.translation.x
    }

    pub fn right(&self) -> f64 {
        self.translation.x + self.scale.x
    }

    pub fn top(&self) -> f64 {
        self.translation.y
    }

    pub fn bottom(&self) -> f64 {
        self.translation.y + self.scale.y
    }

    pub fn intersection_with(&self, other: &Transform) -> bool {
        return f64::max(self.left(), other.left()) < f64::min(self.right(), other.right())
            && f64::max(self.top(), other.top()) < f64::min(self.bottom(), other.bottom());

        // self.left() < other.right() && self.right() > other.left() && self.top() > other.bottom() && self.bottom() < other.top()
    }

    /// Converts translation vector and scale vector to a f64 array
    pub fn into_array(&self) -> [f64; 4] {
        [self.translation.x, self.translation.y, self.scale.x, self.scale.y]
    }
}
