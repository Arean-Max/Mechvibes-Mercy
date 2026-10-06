use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn sync_soundpacks() {
    println!("cargo:rerun-if-changed=soundpacks");

    let (Some(out_dir), Some(manifest_dir)) = (
        std::env::var_os("OUT_DIR").map(PathBuf::from),
        std::env::var_os("CARGO_MANIFEST_DIR").map(PathBuf::from),
    ) else {
        return;
    };

    let Some(profile_dir) = out_dir.ancestors().nth(3) else {
        return;
    };

    if profile_dir.components().any(|c| c.as_os_str() == "dx") {
        return;
    }

    let src = manifest_dir.join("soundpacks");
    if !src.is_dir() {
        return;
    }

    if let Err(e) = copy_changed(&src, &profile_dir.join("soundpacks")) {
        println!("cargo:warning=could not copy soundpacks next to the executable: {e}");
    }
}

fn copy_changed(src: &Path, dst: &Path) -> io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let from = entry.path();
        let to = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_changed(&from, &to)?;
            continue;
        }
        let from_meta = entry.metadata()?;
        let up_to_date = fs::metadata(&to)
            .map(|m| {
                m.len() == from_meta.len() && matches!(
                    (m.modified(), from_meta.modified()),
                    (Ok(a), Ok(b)) if a >= b
                )
            })
            .unwrap_or(false);
        if !up_to_date {
            fs::copy(&from, &to)?;
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    sync_soundpacks();

    #[cfg(windows)]
    {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("assets/icon.ico");
        res.set("ProductName", "Mercy");
        res.set("FileDescription", "Mercy - Mechanical Keyboard Sound Simulator");
        res.set("CompanyName", "Arean Max");
        res.set("LegalCopyright", "Copyright (C) 2026 Arean Max");
        res.compile()?;
    }

    Ok(())
}
