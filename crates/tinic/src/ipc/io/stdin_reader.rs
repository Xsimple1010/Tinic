use std::sync::atomic::Ordering;
use tinic_ipc_protocol::helpers::stdin_reader_trait::StdinReaderTrait;
use tinic_ipc_protocol::input::ProtocolInput;

use crate::ipc::app_state::AppStateHandle;

pub(crate) struct StdinReader;

impl StdinReaderTrait<AppStateHandle> for StdinReader {
    fn event_handle(input: ProtocolInput, state: AppStateHandle) {
        match input {
            ProtocolInput::LoadGame { info } => {
                if state.game_loaded.load(Ordering::SeqCst)
                    && state.game_dispatchers.exit().is_err()
                {
                    println!("Não foi possível parar o jogo atual!");
                    return;
                }

                match state.game_info.lock() {
                    Ok(mut game_info) => {
                        game_info.replace(info);
                    }
                    Err(e) => {
                        println!("Erro ao tentar atualizar o game_info: {e}",)
                    }
                }
            }
            ProtocolInput::GameClose => {
                if !state.game_loaded.load(Ordering::SeqCst) {
                    return;
                }

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

// impl StdinReader {
//     #[doc = "Cria uma thread de leitura de stdin e envia os comandos para o canal tx [tx: Sender<ProtocolInput>"]
//     pub fn start(state: AppStateHandle) {
//         let (tx, rx) = mpsc::channel::<ProtocolInput>();
//
//         std::thread::spawn(move || {
//             let stdin = std::io::stdin();
//             for line in stdin.lock().lines() {
//                 sleep(Duration::from_millis(THREAD_SLEEP_TIME_IN_MILLISECONDS));
//                 match line {
//                     Ok(line) => {
//                         if let Ok(cmd) = serde_json::from_str::<ProtocolInput>(&line) {
//                             let _ = tx.send(cmd);
//                         }
//                     }
//                     Err(_) => break,
//                 }
//             }
//             let _ = tx.send(ProtocolInput::Exit);
//         });
//
//         Self::process_command_thread(rx, state);
//     }
//
//     fn process_command_thread(rx: Receiver<ProtocolInput>, state: AppStateHandle) {
//         std::thread::spawn(move || {
//             loop {
//                 sleep(Duration::from_millis(THREAD_SLEEP_TIME_IN_MILLISECONDS));
//                 if let Ok(cmd) = rx.try_recv() {
//                     match cmd {
//                         ProtocolInput::LoadGame { info } => {
//                             if state.game_loaded.load(Ordering::SeqCst)
//                                 && state.game_dispatchers.exit().is_err()
//                             {
//                                 println!("Não foi possível parar o jogo atual!");
//                                 return;
//                             }
//
//                             match state.game_info.lock() {
//                                 Ok(mut game_info) => {
//                                     game_info.replace(info);
//                                 }
//                                 Err(e) => {
//                                     println!("Erro ao tentar atualizar o game_info: {e}",)
//                                 }
//                             }
//                         }
//                         ProtocolInput::GameClose => {
//                             if !state.game_loaded.load(Ordering::SeqCst) {
//                                 continue;
//                             }
//
//                             if state.game_dispatchers.exit().is_err() {
//                                 println!("Não foi possível parar o jogo atual!");
//                             }
//                         }
//                         ProtocolInput::Exit => {
//                             state.running.store(false, Ordering::SeqCst);
//                             if state.game_dispatchers.exit().is_err() {
//                                 println!("Não foi possível o tinic!");
//                             }
//                         }
//                     }
//                 }
//             }
//         });
//     }
// }
