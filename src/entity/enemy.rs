#[derive(Clone, Copy)]
pub struct Enemy(pub EnemyData);

#[derive(Clone, Copy)]
pub struct EnemyData {
    pub damage: f64,
}

impl Default for EnemyData {
    fn default() -> Self {
        Self { damage: 25.0 }
    }
}
