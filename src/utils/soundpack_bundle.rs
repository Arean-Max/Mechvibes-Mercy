use std::fs;
use std::io::Cursor;
use std::path::Path;
use zip::ZipArchive;

/// Embedded default soundpacks compiled directly into the binary
pub static DEFAULT_SOUNDPACKS_ZIP: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/default_soundpacks.zip"));

/// Checks whether default soundpacks exist in either the built-in directory or custom directory.
/// If neither contains any soundpacks, automatically extracts all embedded default soundpacks
/// into `target_dir` (typically the custom soundpacks directory in AppData).
pub fn ensure_default_soundpacks_extracted(target_dir: &Path) -> Result<usize, String> {
    // If target directory already contains soundpacks, check if keyboard packs exist
    let keyboard_dir = target_dir.join("keyboard");
    if keyboard_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&keyboard_dir) {
            let mut has_packs = false;
            for entry in entries.flatten() {
                if entry.path().is_dir() && entry.path().join("config.json").is_file() {
                    has_packs = true;
                    break;
                }
            }
            if has_packs {
                return Ok(0);
            }
        }
    }

    crate::always_print!(
        "📦 Extracting embedded default soundpacks to: {}",
        target_dir.display()
    );
    extract_embedded_soundpacks(target_dir)
}

/// Extracts embedded soundpacks zip into the specified target directory.
pub fn extract_embedded_soundpacks(target_dir: &Path) -> Result<usize, String> {
    let cursor = Cursor::new(DEFAULT_SOUNDPACKS_ZIP);
    let mut archive = ZipArchive::new(cursor).map_err(|e| format!("Invalid zip archive: {}", e))?;
    let mut extracted_count = 0;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).map_err(|e| format!("Zip entry error: {}", e))?;
        let enclosed_name = file
            .enclosed_name()
            .ok_or_else(|| "Invalid zip path".to_string())?;
        let outpath = target_dir.join(enclosed_name);

        if file.is_dir() {
            fs::create_dir_all(&outpath)
                .map_err(|e| format!("Failed to create dir {}: {}", outpath.display(), e))?;
        } else {
            if let Some(parent) = outpath.parent() {
                if !parent.exists() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create parent dir {}: {}", parent.display(), e))?;
                }
            }

            let should_write = match fs::metadata(&outpath) {
                Ok(m) => m.len() == 0,
                Err(_) => true,
            };

            if should_write {
                let mut outfile = fs::File::create(&outpath)
                    .map_err(|e| format!("Failed to create file {}: {}", outpath.display(), e))?;
                std::io::copy(&mut file, &mut outfile)
                    .map_err(|e| format!("Failed to write file {}: {}", outpath.display(), e))?;
                extracted_count += 1;
            }
        }
    }

    crate::always_print!("✅ Successfully extracted {} default soundpack files", extracted_count);
    Ok(extracted_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn embedded_zip_is_valid_and_non_empty() {
        assert!(!DEFAULT_SOUNDPACKS_ZIP.is_empty(), "Embedded soundpacks zip must not be empty");
        let cursor = Cursor::new(DEFAULT_SOUNDPACKS_ZIP);
        let archive = ZipArchive::new(cursor).expect("Embedded soundpacks zip should be valid");
        assert!(archive.len() > 0, "Embedded archive should contain soundpack files");
    }

    #[test]
    fn embedded_zip_contains_default_packs() {
        let cursor = Cursor::new(DEFAULT_SOUNDPACKS_ZIP);
        let mut archive = ZipArchive::new(cursor).unwrap();
        let mut has_oreo = false;
        let mut has_ping = false;

        for i in 0..archive.len() {
            let file = archive.by_index(i).unwrap();
            let name = file.name().replace('\\', "/");
            if name.contains("keyboard/eg-oreo/config.json") {
                has_oreo = true;
            }
            if name.contains("mouse/ping/config.json") {
                has_ping = true;
            }
        }

        assert!(has_oreo, "Archive must contain keyboard/eg-oreo");
        assert!(has_ping, "Archive must contain mouse/ping");
    }

    #[test]
    fn extracting_into_temporary_dir_succeeds() {
        let temp_dir = std::env::temp_dir().join(format!("mercy_test_sp_{}", uuid::Uuid::new_v4()));
        let count = extract_embedded_soundpacks(&temp_dir).expect("Extraction should succeed");
        assert!(count > 0, "Extracted file count must be greater than 0");
        assert!(temp_dir.join("keyboard").join("eg-oreo").join("config.json").is_file());
        assert!(temp_dir.join("mouse").join("ping").join("config.json").is_file());

        // Subsequent check should detect existing packs and return 0
        let second_run = ensure_default_soundpacks_extracted(&temp_dir).unwrap();
        assert_eq!(second_run, 0, "Should not re-extract when soundpacks already exist");

        let _ = fs::remove_dir_all(&temp_dir);
    }
}

