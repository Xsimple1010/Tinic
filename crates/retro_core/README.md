# Retro Core

**Retro Core** is the module responsible for defining the **core abstractions and interfaces** used to run Libretro cores inside the Tinic ecosystem.

It provides the foundation that allows the rest of Tinic to communicate with emulator cores in a clean and structured way.

---

## 🎯 Purpose

Retro Core acts as the **bridge layer** between the emulator frontend (Tinic) and Libretro cores.

It defines:

- Core lifecycle management  
- Frame execution flow  
- Input handling  
- Audio and video callbacks  
- Environment communication with the core  

---

## 🧩 Responsibilities

Retro Core does **not** handle platform rendering, audio output, or window management directly.  
Instead, it focuses on the **emulation core interface**, leaving platform-specific concerns to other modules.

### It provides:

- 🧠 Core loading and initialization  
- 🔁 Frame execution control  
- 🎮 Input forwarding to the core  
- 📦 Environment callback handling  
- 🔌 A structured API used by higher-level Tinic modules  
