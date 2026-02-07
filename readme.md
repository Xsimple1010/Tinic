<p align="center">
   <img src=".github/tinic_icon.png" alt="Tinic Logo" width="320rem">
</p>

<h1 align="center">Tinic</h1>

<p align="center">
    Tinic is a runtime for Libretro cores that abstracts the native API and provides a ready-to-use foundation for cross-platform frontends.
</p>


# 🎯 What is Tinic for?

Most frontends that implement the Libretro API need to create their own
audio and video layers and also handle controller (gamepad) input events
sent by the user.

And in the worst case, if you are not using C or C++, you will still
need to rely on FFI (Foreign Function Interface) to communicate with the
Libretro API.

All of this makes frontend development much more complex and exhausting
than it should be.\
Shouldn't a frontend simply be a user-friendly interface?

### 👉 The answer is yes.

Handling audio, video implementation, and complex FFI details should not
be the responsibility of the interface layer.\
And that is exactly why **Tinic** was created.

------------------------------------------------------------------------

## 💡 In short

With **Tinic**, you can build your frontend UI in **any language**,
without worrying about the complexity of the Libretro API.

🧠 Tinic handles the hard part.\
🎨 You focus only on the user experience.

------------------------------------------------------------------------

# 🚀 How to use?

Currently, there are **two ways** to use Tinic:

-   **LibTinic** (direct integration in Rust)
-   **Tinic-ipc** (use from other programming languages)

------------------------------------------------------------------------

## 🦀 LibTinic (Rust)

``` rust
fn main() -> Result<(), ErrorHandle> {
    let mut tinic = Tinic::new()?;
    
    // Before continuing, it is necessary to register event listeners (required)
    // See the folder "crates/tinic/examples" for more details
    tinic.set_controller_listener(Box::new(DeviceEventHandle::default()))?;
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

👉 Full code available at:\
**[`crates/tinic/examples/tinic_run.rs`](crates/tinic/examples/tinic_run.rs)**

### 📢 How to communicate with Tinic?

To communicate with Tinic, you need to create a **game_dispatchers**
instance. It is not necessary to have a window open to do this. You can
create **game_dispatchers** once and reuse it for all calls.

``` rust
fn main() -> Result<(), ErrorHandle> {
    let mut tinic = create_tinic()?;
    let dispatch = tinic.get_game_dispatchers();
    
    // Change the current slot (default: 1) to slot 2
    let _ = dispatch.change_default_slot(2);
    
    // Save the current state into slot 2
    let _ = dispatch.save_state(2);

    // Load the saved state from slot 2
    let _ = dispatch.load_state(2);

    // Pause or resume the game
    let _ = dispatch.pause();
    let _ = dispatch.resume();
    
    // Enable or disable the keyboard.
    // By default, when a gamepad is connected, the keyboard will be disabled.
    let _ = dispatch.disable_keyboard();
    let _ = dispatch.enable_keyboard();

    // Get a list of connected devices (gamepads)
    let devices = tinic.retro_controller.unwrap().get_list()?;
    
    // Connect a gamepad
    let _ = dispatch.connect_device(devices[0].clone().into());
    
    // This closes the game window. To create a new window,
    // it is necessary to create a new game_instance
    let _ = dispatch.exit();
}
```

------------------------------------------------------------------------

## 🌐 Tinic-ipc (Other Languages)

As the name suggests, **Tinic-ipc** works as an **IPC (Inter-Process
Communication)** layer between the frontend and the backend.

This means:

✅ No FFI\
✅ No dealing with C/C++\
✅ Simple communication via **JSON**

Instead of complex integrations, your frontend communicates with Tinic
through messages.

📌 Example available at:\
**Retronic (frontend using Tinic-ipc)**\
https://github.com/Xsimple1010/retronic/tree/master/native

------------------------------------------------------------------------

# 🔨 Supporting Tools

Like **RetroArch**, Tinic also requires external files such as:

-   **RDB** (database containing ROM collections)
-   Thumbnails
-   Core information files
-   And of course, save states

### 🗂️ Tinic Super

**Tinic Super** is the module responsible for managing all external
resources and metadata used by **Tinic**.\
See the [Readme here](./crates/tinic_super/readme.md)

### 🗄️ Tinic Database

**Tinic Database** is a module created to make working with game
databases much easier for frontend developers.\
See the [Readme here](./crates/tinic_database/readme.md)
