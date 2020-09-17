use super::LifeSystem;

#[derive(Debug)]
pub struct SnakeLifeSystem {}

impl SnakeLifeSystem {
    pub fn new() -> Self {
        Self {}
    }
}

impl LifeSystem for SnakeLifeSystem {
    fn is_alive(&self) -> bool {
        true
    }

    fn hit(&mut self) {}

    fn update(&mut self) {}
}
