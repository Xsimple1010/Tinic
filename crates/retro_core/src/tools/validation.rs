use generics::constants::INVALID_CONTROLLER_PORT;
use generics::error_handle::ErrorHandle;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

/// Input validation utilities for retro_core
pub struct InputValidator;

impl InputValidator {
    /// Validate file path and ensure it exists
    pub fn validate_file_path(path: &str) -> Result<PathBuf, ErrorHandle> {
        if path.is_empty() {
            return Err(ErrorHandle::new("File path cannot be empty"));
        }

        // Check for null bytes that could cause issues with C interop
        if path.contains('\0') {
            return Err(ErrorHandle::new("File path contains null bytes"));
        }

        // Check for excessively long paths
        if path.len() > 4096 {
            return Err(ErrorHandle::new(
                "File path is too long (max 4096 characters)",
            ));
        }

        let path_buf = PathBuf::from(path);

        // Validate that path doesn't contain suspicious patterns
        Self::check_path_traversal(&path_buf)?;

        // Canonicalize to resolve any relative components
        let canonical_path = path_buf
            .canonicalize()
            .map_err(|e| ErrorHandle::new(&format!("Invalid file path '{}': {}", path, e)))?;

        if !canonical_path.exists() {
            return Err(ErrorHandle::new(&format!("File does not exist: {}", path)));
        }

        if !canonical_path.is_file() {
            return Err(ErrorHandle::new(&format!("Path is not a file: {}", path)));
        }

        Ok(canonical_path)
    }

    /// Validate directory path
    pub fn validate_directory_path(path: &str) -> Result<PathBuf, ErrorHandle> {
        if path.is_empty() {
            return Err(ErrorHandle::new("Directory path cannot be empty"));
        }

        if path.contains('\0') {
            return Err(ErrorHandle::new("Directory path contains null bytes"));
        }

        if path.len() > 4096 {
            return Err(ErrorHandle::new(
                "Directory path is too long (max 4096 characters)",
            ));
        }

        let path_buf = PathBuf::from(path);
        Self::check_path_traversal(&path_buf)?;

        // For directories, we might need to create them, so don't require existence
        let canonical_path = if path_buf.exists() {
            path_buf.canonicalize().map_err(|e| {
                ErrorHandle::new(&format!("Invalid directory path '{}': {}", path, e))
            })?
        } else {
            // If it doesn't exist, validate parent exists and is accessible
            if let Some(parent) = path_buf.parent()
                && parent.exists()
                && !parent.is_dir()
            {
                return Err(ErrorHandle::new(&format!(
                    "Parent path exists but is not a directory: {}",
                    parent.display()
                )));
            }
            path_buf
        };

        Ok(canonical_path)
    }

    /// Check for path traversal attacks
    fn check_path_traversal(path: &Path) -> Result<(), ErrorHandle> {
        let path_str = path.to_string_lossy();

        // Check for common path traversal patterns
        let dangerous_patterns = ["..", "//", "\\\\", "..\\", "../", "\\..", "/."];

        for pattern in &dangerous_patterns {
            if path_str.contains(pattern) {
                return Err(ErrorHandle::new(&format!(
                    "Path contains potentially dangerous pattern '{}': {}",
                    pattern, path_str
                )));
            }
        }

        Ok(())
    }

    /// Validate ROM file extension
    pub fn validate_rom_extension(
        path: &Path,
        valid_extensions: &String,
    ) -> Result<(), ErrorHandle> {
        let extension = path
            .extension()
            .ok_or_else(|| ErrorHandle::new("File has no extension"))?
            .to_string_lossy()
            .to_lowercase();

        let valid_exts = valid_extensions
            .split('|')
            .map(|s| s.trim().to_lowercase())
            .collect::<Vec<String>>();

        if !valid_exts.contains(&extension) {
            return Err(ErrorHandle::new(&format!(
                "Invalid ROM extension '{}'. Valid extensions: {}",
                extension, valid_extensions
            )));
        }

        Ok(())
    }

    /// Validate file size is within reasonable limits
    pub fn validate_file_size(path: &Path, max_size_mb: u64) -> Result<u64, ErrorHandle> {
        let metadata = path
            .metadata()
            .map_err(|e| ErrorHandle::new(&format!("Cannot read file metadata: {}", e)))?;

        let size = metadata.len();
        let max_size_bytes = max_size_mb * 1024 * 1024;

        if size > max_size_bytes {
            return Err(ErrorHandle::new(&format!(
                "File size {} MB exceeds maximum allowed size {} MB",
                size / (1024 * 1024),
                max_size_mb
            )));
        }

        if size == 0 {
            return Err(ErrorHandle::new("File is empty"));
        }

        Ok(size)
    }

    /// Safely create C string from Rust string
    pub fn create_safe_c_string(s: &str, err_message: &str) -> Result<CString, ErrorHandle> {
        if s.is_empty() {
            return Err(ErrorHandle::new(&format!(
                "{err_message}: Cannot create C string from empty string"
            )));
        }

        if s.len() > 65535 {
            return Err(ErrorHandle::new(&format!(
                "{err_message}: String too long for C string conversion"
            )));
        }

        CString::new(s).map_err(|e| ErrorHandle::new(&format!("Failed to create C string: {}", e)))
    }

    /// Safely read C string from raw pointer with bounds checking
    pub unsafe fn read_safe_c_string(
        ptr: *const c_char,
        max_len: usize,
    ) -> Result<String, ErrorHandle> {
        if ptr.is_null() {
            return Err(ErrorHandle::new("Cannot read from null pointer"));
        }

        // Use CStr::from_ptr but with additional safety checks
        let c_str = unsafe { CStr::from_ptr(ptr) };
        let bytes = c_str.to_bytes();

        if bytes.len() > max_len {
            return Err(ErrorHandle::new(&format!(
                "C string length {} exceeds maximum allowed length {}",
                bytes.len(),
                max_len
            )));
        }

        c_str
            .to_str()
            .map(|s| s.to_owned())
            .map_err(|e| ErrorHandle::new(&format!("Invalid UTF-8 in C string: {}", e)))
    }

    /// Validate controller port number
    pub fn validate_controller_port(port: i16) -> Result<u16, ErrorHandle> {
        if port <= INVALID_CONTROLLER_PORT {
            return Err(ErrorHandle::new("Controller port cannot be negative"));
        }

        if port > 7 {
            return Err(ErrorHandle::new("Controller port cannot exceed 7"));
        }

        Ok(port as u16)
    }

    /// Validate memory buffer size
    pub fn validate_buffer_size(size: usize, max_size: usize) -> Result<(), ErrorHandle> {
        if size == 0 {
            return Err(ErrorHandle::new("Buffer size cannot be zero"));
        }

        if size > max_size {
            return Err(ErrorHandle::new(&format!(
                "Buffer size {} exceeds maximum allowed size {}",
                size, max_size
            )));
        }

        Ok(())
    }

    /// Validate raw pointer is not null
    pub fn validate_non_null_ptr<T>(ptr: *const T, name: &str) -> Result<(), ErrorHandle> {
        if ptr.is_null() {
            return Err(ErrorHandle::new(&format!("{} pointer is null", name)));
        }
        Ok(())
    }

    /// Validate raw mutable pointer is not null
    pub fn validate_non_null_mut_ptr<T>(ptr: *mut T, name: &str) -> Result<(), ErrorHandle> {
        if ptr.is_null() {
            return Err(ErrorHandle::new(&format!("{} pointer is null", name)));
        }
        Ok(())
    }

    /// Sanitize string for safe logging (remove control characters)
    pub fn sanitize_for_logging(s: &str) -> String {
        s.chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\t')
            .collect()
    }

    /// Validate audio sample rate
    pub fn validate_sample_rate(sample_rate: u32) -> Result<(), ErrorHandle> {
        match sample_rate {
            8000..=192000 => Ok(()),
            _ => Err(ErrorHandle::new(&format!(
                "Invalid sample rate {}. Must be between 8000 and 192000 Hz",
                sample_rate
            ))),
        }
    }

    /// Validate save state slot number
    pub fn validate_save_slot(slot: usize) -> Result<(), ErrorHandle> {
        if slot > 99 {
            return Err(ErrorHandle::new("Save slot cannot exceed 99"));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use generics::test_workdir::get_test_rom_path;
    use std::fs::File;
    use tempfile::TempDir;

    #[test]
    fn test_validate_file_path_success() {
        let rom_path = get_test_rom_path();
        let result = InputValidator::validate_file_path(&rom_path.display().to_string());
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_file_path_nonexistent() {
        let result = InputValidator::validate_file_path("/nonexistent/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_file_path_empty() {
        let result = InputValidator::validate_file_path("");
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_file_path_null_bytes() {
        let result = InputValidator::validate_file_path("test\0file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_check_path_traversal() {
        let dangerous_paths = [
            "../../../etc/passwd",
            "test/../../../etc/passwd",
            "..\\..\\windows\\system32",
        ];

        for path in &dangerous_paths {
            let path_buf = PathBuf::from(path);
            let result = InputValidator::check_path_traversal(&path_buf);
            assert!(result.is_err(), "Should reject dangerous path: {}", path);
        }
    }

    #[test]
    fn test_validate_rom_extension() {
        let temp_dir = TempDir::new().unwrap();
        let rom_path = temp_dir.path().join("game.sfc");
        File::create(&rom_path).unwrap();

        let result = InputValidator::validate_rom_extension(&rom_path, &"sfc|smc|fig".to_string());
        assert!(result.is_ok());

        let result = InputValidator::validate_rom_extension(&rom_path, &"nes|gb|gbc".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_controller_port() {
        assert!(InputValidator::validate_controller_port(0).is_ok());
        assert!(InputValidator::validate_controller_port(3).is_ok());
        assert!(InputValidator::validate_controller_port(7).is_ok());

        assert!(InputValidator::validate_controller_port(-1).is_err());
        assert!(InputValidator::validate_controller_port(8).is_err());
    }

    #[test]
    fn test_validate_sample_rate() {
        assert!(InputValidator::validate_sample_rate(44100).is_ok());
        assert!(InputValidator::validate_sample_rate(48000).is_ok());

        assert!(InputValidator::validate_sample_rate(7999).is_err());
        assert!(InputValidator::validate_sample_rate(192001).is_err());
    }

    #[test]
    fn test_create_safe_c_string() {
        assert!(InputValidator::create_safe_c_string("test", "test").is_ok());
        assert!(InputValidator::create_safe_c_string("", "test").is_err());

        let long_string = "a".repeat(70000);
        assert!(InputValidator::create_safe_c_string(&long_string, "test").is_err());
    }

    #[test]
    fn test_sanitize_for_logging() {
        let input = "Normal text\x00\x01\x02with\tcontrol\nchars";
        let sanitized = InputValidator::sanitize_for_logging(input);
        assert_eq!(sanitized, "Normal textwith\tcontrol\nchars");
    }
}
