
# 🗂️ Tinic Super

**Tinic Super** is the module responsible for managing all **external resources and metadata** used by Tinic.

It does not run Libretro cores directly — instead, it acts as the **data management layer**, organizing, downloading, and maintaining everything Tinic needs to function.

---

## 🎯 Main Responsibility

Tinic Super handles **essential files from the Tinic ecosystem**, including:

- 🎮 Libretro cores  
- 🖼 Thumbnails (box art, screenshots, title images)  
- 🗄️ Game databases (RDB)  
- ℹ️ Core information files (`.info`)  
- 💾 (Future) Save states and persistent data  

It ensures these resources are:  
✔ Organized  
✔ Up to date  
✔ Available locally  
✔ Ready for use by Tinic  

---

## 🧩 What It Manages

### 🧠 Libretro Cores
- Downloading compatible cores  
- Organizing them by system/platform  
- Laying the groundwork for version control and future updates  

### 🖼 Thumbnails
- Game box art  
- Stylized title images  
- Screenshots  

### 🗄️ RDB (Retro Database)
Databases containing game metadata such as:
- Official name  
- Developer  
- Release year  
- Genre  
- Region  
- CRC for automatic identification  

### ℹ️ Core `.info` Files
Files that describe cores, including:
- System name  
- Supported extensions  
- BIOS requirements  
- Save state support  
- Other core capabilities  

Tinic Super uses this data so Tinic knows **how to properly handle each core**.

---

## 🌐 Download System

Tinic Super can fetch resources online, such as:

- 📦 Libretro cores  
- 🗄️ RDB files  
- 🖼 Thumbnail packs  
- ℹ️ `.info` files  

---

## 🧱 Directory Structure

Tinic Super defines and manages the standard folder structure used by Tinic:

```

tinic/
├── cores/
├── rdb/
├── thumbnails/
├── info/
├── system/        (BIOS and firmware)
└── saves/         (future)

```

This ensures consistent organization across any platform.

---

## 🔄 Integration with Tinic

Tinic Super provides Tinic with:

| Resource | Usage in Tinic |
|----------|----------------|
| Core | Run games |
| RDB | Identify and display metadata |
| Thumbnails | Visual library interface |
| Info | Know how to configure the core |
| (Future) Save states | Game continuity |

It acts as the **data and support layer**, while Tinic focuses on execution, interface, and user experience.

---

## 💡 Philosophy

Tinic Super exists so that Tinic:

- Doesn’t have to worry about scattered files  
- Keeps everything automatically organized  
- Remains scalable for many systems and games  

It is the **silent infrastructure** that keeps the Tinic ecosystem running smoothly. ⚙️✨
