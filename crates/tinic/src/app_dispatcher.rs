use retro_controllers::RetroGamePad;
use winit::event_loop::{EventLoopClosed, EventLoopProxy};

pub enum GameInstanceActions {
    ConnectDevice(RetroGamePad),
    ChangeDefaultSlot(usize),
    Pause,
    Resume,
    SaveState(usize),
    LoadState(usize),
    DisableKeybaord,
    EnableKeybaord,
    Exit,
}

type GameInstanceActionsClosed = EventLoopClosed<GameInstanceActions>;

#[derive(Debug, Clone)]
pub struct GameInstanceDispatchers {
    proxy: EventLoopProxy<GameInstanceActions>,
}

impl GameInstanceDispatchers {
    pub fn new(proxy: EventLoopProxy<GameInstanceActions>) -> Self {
        Self { proxy }
    }
}

impl GameInstanceDispatchers {
    pub fn exit(&self) -> Result<(), GameInstanceActionsClosed> {
        self.proxy.send_event(GameInstanceActions::Exit)
    }

    pub fn pause(&self) -> Result<(), GameInstanceActionsClosed> {
        self.proxy.send_event(GameInstanceActions::Pause)
    }

    pub fn resume(&self) -> Result<(), GameInstanceActionsClosed> {
        self.proxy.send_event(GameInstanceActions::Resume)
    }

    pub fn load_state(&self, slot: usize) -> Result<(), GameInstanceActionsClosed> {
        self.proxy.send_event(GameInstanceActions::LoadState(slot))
    }

    pub fn save_state(&self, slot: usize) -> Result<(), GameInstanceActionsClosed> {
        self.proxy.send_event(GameInstanceActions::SaveState(slot))
    }

    pub fn disable_keybaord(&self) -> Result<(), GameInstanceActionsClosed> {
        self.proxy.send_event(GameInstanceActions::DisableKeybaord)
    }

    pub fn enable_keybaord(&self) -> Result<(), GameInstanceActionsClosed> {
        self.proxy.send_event(GameInstanceActions::EnableKeybaord)
    }

    pub fn change_default_slot(
        &self,
        slot: usize,
    ) -> Result<(), GameInstanceActionsClosed> {
        self.proxy
            .send_event(GameInstanceActions::ChangeDefaultSlot(slot))
    }

    pub fn connect_device(
        &self,
        device: RetroGamePad,
    ) -> Result<(), GameInstanceActionsClosed> {
        self.proxy
            .send_event(GameInstanceActions::ConnectDevice(device))
    }
}
