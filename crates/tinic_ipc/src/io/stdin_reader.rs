use crate::io::protocol::input::ProtocolInput;
use crate::AppState;
use std::io::BufRead;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use tinic::TinicGameInfo;

pub struct StdinReader {}

impl StdinReader {
    #[doc = "Cria uma thread de leitura de stdin e envia os comandos para o canal tx [tx: Sender<ProtocolInput>"]
    pub fn start(state: Arc<AppState>) {
        let (tx, rx) = mpsc::channel::<ProtocolInput>();

        std::thread::spawn(move || {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                sleep(std::time::Duration::from_secs(THREAD_SLEEP_TIME_IN_SEC));
                match line {
                    Ok(line) => {
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

    fn process_command_thread(rx: Receiver<ProtocolInput>, state: Arc<AppState>) {
        std::thread::spawn(move || {
            loop {
                sleep(std::time::Duration::from_secs(THREAD_SLEEP_TIME_IN_SEC));
                if let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        ProtocolInput::LoadGame {
                            rom_path,
                            core_path,
                            base_retro_path,
                        } => {
                            if state.game_loaded.load(Ordering::SeqCst) {
                                if state.game_dispatchers.exit().is_err() {
                                    println!("Não foi possível parar o jogo atual!");
                                    return;
                                }
                            }

                            match state.game_info.lock() {
                                Ok(mut game_info) => {
                                    game_info.replace(TinicGameInfo {
                                        rom: rom_path,
                                        core: core_path,
                                        sys_dir: base_retro_path,
                                    });
                                }
                                Err(e) => {
                                    println!("Erro ao tentar atualizar o game_info: {e}",)
                                }
                            }
                        }
                        ProtocolInput::GameClose => {
                            if state.game_dispatchers.exit().is_err() {
                                println!("Não foi possível parar o jogo atual!");
                            }
                        }
                        ProtocolInput::Exit => {
                            state.running.store(false, Ordering::SeqCst);
                            if state.game_dispatchers.exit().is_err() {
                                println!("Não foi possível o tinic!");
                            }
                        }
                    }
                }
            }
            println!("command thread closed");
        });
    }
}
