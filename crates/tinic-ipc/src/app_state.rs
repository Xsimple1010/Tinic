use std::sync::atomic::AtomicBool;
use std::sync::{Arc, Mutex};
use tinic::{GameInstanceDispatchers, TinicGameInfo};

pub struct AppState {
    pub game_dispatchers: Arc<GameInstanceDispatchers>,
    pub game_loaded: AtomicBool,
    pub running: AtomicBool,
    pub game_info: Mutex<Option<TinicGameInfo>>,
}

pub type AppStateHandle = Arc<AppState>;

impl AppState {
    pub fn new(game_dispatchers: GameInstanceDispatchers) -> AppStateHandle {
        Arc::new(Self {
            game_dispatchers: Arc::new(game_dispatchers),
            game_loaded: AtomicBool::new(false),
            running: AtomicBool::new(true),
            game_info: Mutex::new(None),
        })
    }
}
