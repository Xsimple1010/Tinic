<p align="center">
   <img src=".github/tinic_icon.png" alt="Tinic Logo" width="320rem">
</p>

<h1 align="center">Tinic</h1>

<p align="center">
    Tinic é um runtime para núcleos Libretro que abstrai a API nativa e fornece uma base pronta para 
    frontends multiplataforma.
</p>

# 🎯 Qual a finalidade do Tinic?

A maioria dos frontends que implementam a API Libretro precisa criar suas próprias camadas de áudio, vídeo e também 
lidar com os eventos de controles (gamepads) enviados pelo usuário.

E no pior cenário, se você não estiver usando C ou C++, ainda vai precisar recorrer a FFI (Foreign Function Interface)
para se comunicar com a API Libretro.

Tudo isso torna o desenvolvimento de um frontend muito mais complexo e cansativo do que deveria ser.
Um frontend não deveria ser apenas uma interface amigável para o usuário?

### 👉 A resposta é sim.

Lidar com implementação de áudio, vídeo e detalhes complexos de FFI não
deveria ser responsabilidade da camada de interface.\
E foi exatamente para resolver esse problema que o **Tinic** foi criado.

------------------------------------------------------------------------

## 💡 Em poucas palavras

Com o **Tinic**, você pode criar a UI do seu frontend em **qualquer
linguagem**, sem precisar se preocupar com a complexidade da API
Libretro.

🧠 O Tinic cuida da parte difícil.\
🎨 Você foca apenas na experiência do usuário.

------------------------------------------------------------------------

# 🚀 Como usar?

Atualmente existem **duas formas** de usar o Tinic:

-   **LibTinic** (integração direta em Rust)
-   **Tinic-ipc** (uso a partir de outras linguagens)

------------------------------------------------------------------------

## 🦀 LibTinic (Rust)

``` rust
fn main() -> Result<(), ErrorHandle> {
    let mut tinic = Tinic::new()?;
    
    // Antes de continuar é preciso registrar os listeners de eventos (obrigatório)
    // Veja a pasta "crates/tinic/examples" para mais detalhes
    tinic.set_controle_listener(Box::new(DeviceEventHandle::default()))?;
    tinic.set_window_listener(Box::new(WindowEvents));

    let test_dir = "tinic_example";

    let game_info = TinicGameInfo {
        core: get_test_core_path().display().to_string(),
        rom: get_test_rom_path().display().to_string(),
        sys_dir: create_test_work_dir_path(test_dir).display().to_string(),
    };

    let game_instance = tinic.create_game_instance(game_info)?;
    tinic.run(game_instance)?;

    remove_test_work_dir_path(test_dir)
}
```

👉 Código completo disponível em:\
**[`crates/tinic/examples/tinic_run.rs`](crates/tinic/examples/tinic_run.rs)**

### 📢 Como se comunicar com o Tinic?

Para se comunicar com Tinic você precisa criar uma **game_dispatchers**. Não é necessário ter uma janela 
aberta para isso! Então você pode criar o **game_dispatchers** uma unica vez e usar para todas as chamadas.

``` rust 
   fn main() -> Result<(), ErrorHandle> {
        let mut tinic = create_tinic()?;
        let dispatch = tinic.get_game_dispatchers();
        
        // troca o slot atual(default: 1) para o slot 2
        let _ = dispatch.change_default_slot(2);
        
        // salva o state atual no slot 2
        let _ = dispatch.save_state(2);
    
        // carrega o state salvo no slot 2
        let _ = dispatch.load_state(2);
    
        // pausa ou resulme o jogo
        let _ = dispatch.pause();
        let _ = dispatch.resume();
        
        // habilita ou desabilita o teclado, 
        // por padrão ao conectar uma gamepad o teclado será desabilitado
        let _ = dispatch.disable_keyboard();
        let _ = dispatch.enable_keyboard();
    
        // pega uma lista de dispositivos(gamepads) conectados
        let devices = tinic.retro_controle.unwrap().get_list()?;
        
        // conecta um gamepad
        let _ = dispatch.connect_device(devices[0].clone().into());
        
        // isso fecha a janela do jogo, para criar uma nova janela é necessario
        // criar uma nova game_instance
        let _ = dispatch.exit();
   }
```

------------------------------------------------------------------------

## 🌐 Tinic-ipc (Outras linguagens)

Como o nome sugere, o **Tinic-ipc** funciona como uma camada de **IPC
(Inter-Process Communication)** entre o frontend e o backend.

Isso significa:

✅ Sem FFI\
✅ Sem lidar com C/C++\
✅ Comunicação simples via **JSON**

Em vez de integrações complexas, o seu frontend conversa com o Tinic por
mensagens.

📌 Exemplo disponível em:\
**Retronic (frontend usando Tinic-ipc)**\
https://github.com/Xsimple1010/retronic/tree/master/native

