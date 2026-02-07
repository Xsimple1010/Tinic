use crate::system::SysInfo;
use crate::tools::validation::InputValidator;
use generics::constants::SAVE_EXTENSION_FILE;
use generics::error_handle::ErrorHandle;
use libretro_sys::binding_libretro::{LibretroRaw, retro_game_info};
use std::fs;
use std::sync::Arc;
use std::{
    ffi::CString,
    fs::File,
    io::Read,
    os::raw::c_void,
    path::{Path, PathBuf},
    ptr::null,
};

/// Maximum ROM size in MB (500MB should be enough for most cases)
const MAX_ROM_SIZE_MB: u64 = 500;

/// Maximum save state size in MB (100MB should be more than enough)
const MAX_SAVE_STATE_SIZE_MB: u64 = 100;

/// Maximum number of save slots
const MAX_SAVE_SLOTS: usize = 99;

/// Safer version of SaveInfo with additional validation
pub struct SaveInfo<'a> {
    pub save_dir: &'a str,
    pub library_name: &'a str,
    pub rom_name: &'a str,
    pub slot: usize,
    pub buffer_size: usize,
}

/// Create a rom info wrapper
struct RomInfo {
    data: Vec<u8>,
    meta: CString,
    path: CString,
    size: usize,
}

impl RomInfo {
    pub fn to_core_native(&self) -> retro_game_info {
        retro_game_info {
            data: if self.data.is_empty() {
                null()
            } else {
                self.data.as_ptr() as *const c_void
            },
            meta: self.meta.as_ptr(),
            path: self.path.as_ptr(),
            size: self.size,
        }
    }
}

impl<'a> SaveInfo<'a> {
    /// Create new SafeSaveInfo with validation
    pub fn new(
        save_dir: &'a str,
        library_name: &'a str,
        rom_name: &'a str,
        slot: usize,
        buffer_size: usize,
    ) -> Result<Self, ErrorHandle> {
        if slot > MAX_SAVE_SLOTS {
            return Err(ErrorHandle::new(&format!(
                "Invalid slot number: {}. Maximum allowed slot number is {}",
                slot, MAX_SAVE_SLOTS
            )));
        }

        // Validate save directory
        InputValidator::validate_directory_path(save_dir)?;

        // Validate slot number
        InputValidator::validate_save_slot(slot)?;

        // Validate buffer size (max 100MB)
        InputValidator::validate_buffer_size(
            buffer_size,
            MAX_SAVE_STATE_SIZE_MB as usize * 1024 * 1024,
        )?;

        // Sanitize names for safe file system operations
        if library_name.is_empty() || rom_name.is_empty() {
            return Err(ErrorHandle::new(
                "Library name and ROM name cannot be empty",
            ));
        }

        // Check for dangerous characters in names
        for name in [library_name, rom_name] {
            if name.contains(['/', '\\', ':', '*', '?', '"', '<', '>', '|', '\0']) {
                return Err(ErrorHandle::new(&format!(
                    "Invalid characters in name: {}. Names cannot contain filesystem reserved characters",
                    InputValidator::sanitize_for_logging(name)
                )));
            }
        }

        Ok(Self {
            save_dir,
            library_name,
            rom_name,
            slot,
            buffer_size,
        })
    }
}

/// Safer ROM loading and save state management
pub struct RomTools;

impl RomTools {
    /// Safely load a ROM with comprehensive validation
    pub fn try_load_game(
        libretro_raw: &Arc<LibretroRaw>,
        sys_info: &SysInfo,
        path: &str,
    ) -> Result<bool, ErrorHandle> {
        // Validate and canonicalize the ROM path
        let validated_path = InputValidator::validate_file_path(path)?;

        // Validate ROM extension
        InputValidator::validate_rom_extension(&validated_path, &sys_info.valid_extensions)?;

        // Create validated ROM info
        let rom_info = Self::create_game_info(&validated_path, sys_info)?;
        let native_core_info = rom_info.to_core_native();

        // Load the game using the validated info
        let loaded = unsafe { libretro_raw.retro_load_game(&native_core_info) };

        if !loaded {
            return Err(ErrorHandle::new(&format!(
                "Core rejected ROM file: {}",
                validated_path.display()
            )));
        }

        Ok(loaded)
    }

    /// Create retro_game_info with proper validation and memory management
    fn create_game_info(path: &Path, sys_info: &SysInfo) -> Result<RomInfo, ErrorHandle> {
        let mut buf: Vec<u8> = Vec::new();
        let meta = CString::new("")?;

        // Create safe C string for path
        let path_str = path
            .to_str()
            .ok_or_else(|| ErrorHandle::new("Path contains invalid UTF-8 characters"))?;
        let path_cstring =
            InputValidator::create_safe_c_string(path_str, "Cannot send ROM path to core")?;

        let mut size = 0;

        // Only read file into memory if the core doesn't need the full path
        if !*sys_info.need_full_path {
            // Validate file size
            let file_size = InputValidator::validate_file_size(&path, MAX_ROM_SIZE_MB)?;

            // Additional safety check before reading large files into memory
            if file_size > 100 * 1024 * 1024 {
                // For files > 100MB, confirm this is intentional
                println!(
                    "Warning: Loading large ROM file ({} MB) into memory",
                    file_size / (1024 * 1024)
                );
            }

            Self::validate_rom_integrity(path)?;

            let mut file = File::open(path)
                .map_err(|e| ErrorHandle::new(&format!("Failed to open ROM file: {}", e)))?;

            size = file_size as usize;
            buf = Vec::with_capacity(size);

            file.read_to_end(&mut buf)
                .map_err(|e| ErrorHandle::new(&format!("Failed to read ROM file: {}", e)))?;

            // Verify we read the expected amount
            if buf.len() != size {
                return Err(ErrorHandle::new(&format!(
                    "File size mismatch: expected {}, read {}",
                    size,
                    buf.len()
                )));
            }
        }

        let game_info = RomInfo {
            data: buf,
            meta,
            path: path_cstring,
            size,
        };

        Ok(game_info)
    }

    /// Safely extract ROM name with validation
    pub fn get_rom_name(path: &Path) -> Result<String, ErrorHandle> {
        let file_name = path
            .file_name()
            .ok_or_else(|| ErrorHandle::new("Cannot extract filename from path"))?
            .to_str()
            .ok_or_else(|| ErrorHandle::new("Filename contains invalid UTF-8 characters"))?;

        let extension = path
            .extension()
            .ok_or_else(|| ErrorHandle::new("File has no extension"))?
            .to_str()
            .ok_or_else(|| ErrorHandle::new("Extension contains invalid UTF-8 characters"))?;

        let extension_with_dot = format!(".{}", extension);
        let name = file_name.replace(&extension_with_dot, "");

        // Validate the extracted name
        if name.is_empty() {
            return Err(ErrorHandle::new(
                "ROM name cannot be empty after removing extension",
            ));
        }

        if name.len() > 255 {
            return Err(ErrorHandle::new(
                "ROM name is too long (max 255 characters)",
            ));
        }

        // Sanitize the name for filesystem safety
        let sanitized_name = Self::sanitize_filename(&name);

        Ok(sanitized_name)
    }

    /// Sanitize filename for safe filesystem operations
    fn sanitize_filename(name: &str) -> String {
        name.chars()
            .map(|c| {
                match c {
                    // Replace dangerous characters with underscores
                    '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
                    // Keep control characters as underscores
                    c if c.is_control() => '_',
                    // Keep everything else
                    c => c,
                }
            })
            .collect()
    }

    /// Safely create save state with comprehensive validation
    pub fn create_save_state<CA>(save_info: SaveInfo, get_data: CA) -> Result<PathBuf, ErrorHandle>
    where
        CA: FnOnce(&mut Vec<u8>, usize) -> bool,
    {
        // Validate buffer size one more time
        if save_info.buffer_size == 0 {
            return Err(ErrorHandle::new("Save state buffer size cannot be zero"));
        }

        if save_info.buffer_size > MAX_SAVE_STATE_SIZE_MB as usize * 1024 * 1024 {
            return Err(ErrorHandle::new(&format!(
                "Save state size {} MB exceeds maximum allowed size {} MB",
                save_info.buffer_size / (1024 * 1024),
                MAX_SAVE_STATE_SIZE_MB
            )));
        }

        // Create buffer with proper initialization
        let mut data = vec![0u8; save_info.buffer_size];

        // Call the core's serialize function
        let success = get_data(&mut data, save_info.buffer_size);

        if !success {
            return Err(ErrorHandle::new("Core failed to serialize save state"));
        }

        // Get validated save path
        let save_path = Self::get_validated_save_path(&save_info)?;

        // Ensure parent directory exists
        if let Some(parent) = save_path.parent() {
            fs::create_dir_all(parent).map_err(|e| {
                ErrorHandle::new(&format!("Failed to create save directory: {}", e))
            })?;
        }

        // Write to temporary file first, then rename (atomic operation)
        let temp_path = save_path.with_extension("tmp");

        fs::write(&temp_path, &data).map_err(|e| {
            ErrorHandle::new(&format!(
                "Failed to write save state to temporary file: {}",
                e
            ))
        })?;

        // Atomic rename
        fs::rename(&temp_path, &save_path).map_err(|e| {
            // Clean up temp file if rename fails
            let _ = fs::remove_file(&temp_path);
            ErrorHandle::new(&format!("Failed to finalize save state file: {}", e))
        })?;

        // Verify the file was written correctly
        let written_size = fs::metadata(&save_path)
            .map_err(|e| ErrorHandle::new(&format!("Failed to verify saved file: {}", e)))?
            .len();

        if written_size != data.len() as u64 {
            return Err(ErrorHandle::new(&format!(
                "Save state file size mismatch: expected {}, got {}",
                data.len(),
                written_size
            )));
        }

        Ok(save_path)
    }

    /// Safely load save state with validation
    pub fn load_save_state<CA>(save_info: SaveInfo, send_to_core: CA) -> Result<(), ErrorHandle>
    where
        CA: FnOnce(&mut Vec<u8>, usize) -> bool,
    {
        let save_path = Self::get_validated_save_path(&save_info)?;

        if !save_path.exists() {
            return Err(ErrorHandle::new(&format!(
                "Save state file does not exist: {}",
                save_path.display()
            )));
        }

        // Validate file before reading
        let file_size = InputValidator::validate_file_size(&save_path, MAX_SAVE_STATE_SIZE_MB)?;

        // Read and validate the save state data
        let mut data = fs::read(&save_path)
            .map_err(|e| ErrorHandle::new(&format!("Failed to read save state file: {}", e)))?;

        let actual_size = data.len();

        // Validate size consistency
        if actual_size != file_size as usize {
            return Err(ErrorHandle::new(&format!(
                "Save state file size inconsistency: metadata says {}, actual {}",
                file_size, actual_size
            )));
        }

        // Check if save state is compatible with current core buffer size
        if actual_size > save_info.buffer_size {
            return Err(ErrorHandle::new(&format!(
                "Save state file size {} exceeds core buffer size {}. Incompatible save state.",
                actual_size, save_info.buffer_size
            )));
        }

        // Pad with zeros if necessary (some cores might expect exact buffer size)
        if actual_size < save_info.buffer_size {
            data.resize(save_info.buffer_size, 0);
        }

        // Send to core for deserialization
        let success = send_to_core(&mut data, actual_size);

        if !success {
            return Err(ErrorHandle::new("Core failed to deserialize save state"));
        }

        Ok(())
    }

    /// Get validated save path with proper directory structure
    fn get_validated_save_path(save_info: &SaveInfo) -> Result<PathBuf, ErrorHandle> {
        let base_path = InputValidator::validate_directory_path(save_info.save_dir)?;

        // Create sanitized subdirectory structure
        let library_subdir = Self::sanitize_filename(save_info.library_name);
        let rom_subdir = Self::sanitize_filename(save_info.rom_name);

        let mut path = base_path;
        path.push(library_subdir);
        path.push(rom_subdir);

        // Validate the constructed path isn't too deep or too long
        let path_str = path.to_string_lossy();
        if path_str.len() > 1024 {
            return Err(ErrorHandle::new("Save path is too long"));
        }

        // Create the filename with validation
        let file_name = format!("{:02}.{}", save_info.slot, SAVE_EXTENSION_FILE);
        path.push(file_name);

        Ok(path)
    }

    /// Validate ROM file integrity (basic checks)
    pub fn validate_rom_integrity(path: &Path) -> Result<(), ErrorHandle> {
        let file = File::open(path).map_err(|e| {
            ErrorHandle::new(&format!("Cannot open ROM file for integrity check: {}", e))
        })?;

        let metadata = file
            .metadata()
            .map_err(|e| ErrorHandle::new(&format!("Cannot read ROM file metadata: {}", e)))?;

        // Check if file is actually a file and not a directory/symlink
        if !metadata.is_file() {
            return Err(ErrorHandle::new(
                "ROM path does not point to a regular file",
            ));
        }

        // Check file permissions (readable)
        let permissions = metadata.permissions();
        if permissions.readonly() {
            println!("Warning: ROM file is read-only");
        }

        // Basic size sanity check
        if metadata.len() == 0 {
            return Err(ErrorHandle::new("ROM file is empty"));
        }

        if metadata.len() > (MAX_ROM_SIZE_MB * 1024 * 1024) {
            return Err(ErrorHandle::new(&format!(
                "ROM file is too large: {} MB (max {} MB)",
                metadata.len() / (1024 * 1024),
                MAX_ROM_SIZE_MB
            )));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::TempDir;

    #[test]
    fn test_safe_save_info_creation() {
        let result = SaveInfo::new("/tmp", "test_core", "test_rom", 5, 1024);
        assert!(result.is_ok());

        // Test invalid slot
        let result = SaveInfo::new("/tmp", "test_core", "test_rom", 100, 1024);
        assert!(result.is_err());

        // Test empty names
        let result = SaveInfo::new("/tmp", "", "test_rom", 5, 1024);
        assert!(result.is_err());
    }

    #[test]
    fn test_sanitize_filename() {
        assert_eq!(RomTools::sanitize_filename("test<>file"), "test__file");
        assert_eq!(RomTools::sanitize_filename("normal_name"), "normal_name");
    }

    #[test]
    fn test_get_rom_name() {
        let path = Path::new("test_game.sfc");
        let result = RomTools::get_rom_name(path);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_game");

        // Test path without extension
        let path = Path::new("no_extension");
        let result = RomTools::get_rom_name(path);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_rom_integrity() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.rom");

        // Create a valid test file
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"test rom data").unwrap();

        let result = RomTools::validate_rom_integrity(&file_path);
        assert!(result.is_ok());

        // Test empty file
        let empty_path = temp_dir.path().join("empty.rom");
        File::create(&empty_path).unwrap();
        let result = RomTools::validate_rom_integrity(&empty_path);
        assert!(result.is_err());
    }
}
