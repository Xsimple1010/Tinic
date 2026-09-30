use crate::input::ProtocolInput;
use std::{
    io::BufRead,
    sync::mpsc::{self, Receiver},
    thread::sleep,
    time::Duration,
};

pub const THREAD_SLEEP_TIME_IN_MILLISECONDS: u64 = 16;

pub trait StdinReaderTrait<S: Clone + Sync + Send + 'static> {
    fn start(state: S) {
        let (tx, rx) = mpsc::channel::<ProtocolInput>();

        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                sleep(Duration::from_millis(THREAD_SLEEP_TIME_IN_MILLISECONDS));
                match line {
                    Ok(line) => {
                        println!("{line:?}");
                        if let Ok(cmd) = serde_json::from_str::<ProtocolInput>(&line) {
                            let _ = tx.send(cmd);
                        }
                    }
                    Err(_) => break,
                }
            }
            let _ = tx.send(ProtocolInput::Exit);
        });

        Self::process_command_thread(rx, state);
    }

    fn process_command_thread(rx: Receiver<ProtocolInput>, state: S) {
        std::thread::spawn(move || {
            loop {
                sleep(Duration::from_millis(THREAD_SLEEP_TIME_IN_MILLISECONDS));
                if let Ok(cmd) = rx.try_recv() {
                    Self::event_handle(cmd, state.clone());
                }
            }
        });
    }

    fn event_handle(input: ProtocolInput, state: S);
}
