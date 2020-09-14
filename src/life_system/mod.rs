pub trait LifeSystem
where
    Self: std::fmt::Debug,
{
    fn is_alive(&self) -> bool;
}
