use super::LifeSystem;

#[derive(Debug)]
pub struct SwordLifeSystem {
    bounces: u8,
    breaks_at: u8,
}

impl SwordLifeSystem {
    pub fn new() -> SwordLifeSystem {
        SwordLifeSystem {
            bounces: 0,
            breaks_at: 7,
        }
    }
}

impl LifeSystem for SwordLifeSystem {
    fn is_alive(&self) -> bool {
        self.bounces < self.breaks_at
    }

    fn hit(&mut self) {
        self.bounces += 1;
    }

    fn update(&mut self) {}
}
