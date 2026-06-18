use nexgit_core::Core;

#[derive(Debug, Clone)]
pub struct ServerState {
    core: Core,
}

impl ServerState {
    pub fn new() -> Self {
        Self { core: Core::new() }
    }

    pub fn core(&self) -> &Core {
        &self.core
    }
}
