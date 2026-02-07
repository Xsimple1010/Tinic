pub trait WindowListener {
    fn window_closed(&self);

    fn window_opened(&self);

    fn game_loaded_result(&self, suss: bool);

    fn game_closed(&self);

    fn game_paused(&self);

    fn game_resumed(&self);

    fn save_state_result(&self, suss: bool);

    fn load_state_result(&self, suss: bool);

    fn keyboard_state(&self, has_using: bool);
}
