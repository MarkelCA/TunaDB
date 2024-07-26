use core::storage::Engine;

pub trait SendableEngine: Engine + Send + Sync {}

impl<T: Engine + Send + Sync> SendableEngine for T {}
