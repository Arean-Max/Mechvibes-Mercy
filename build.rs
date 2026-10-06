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

#[cfg(windows)]
fn sync_webview2_loader() {
    println!("cargo:rerun-if-changed=assets/windows/WebView2Loader.dll");

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

    let src = manifest_dir
        .join("assets")
        .join("windows")
        .join("WebView2Loader.dll");
    if !src.is_file() {
        return;
    }

    let dst = profile_dir.join("WebView2Loader.dll");
    let up_to_date = match (fs::metadata(&src), fs::metadata(&dst)) {
        (Ok(s), Ok(d)) => s.len() == d.len(),
        _ => false,
    };

    if !up_to_date {
        let _ = fs::copy(&src, &dst);
    }
}

fn package_default_soundpacks() -> io::Result<()> {
    println!("cargo:rerun-if-changed=soundpacks");

    let out_dir = PathBuf::from(
        std::env::var("OUT_DIR").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    );
    let zip_path = out_dir.join("default_soundpacks.zip");

    let manifest_dir = PathBuf::from(
        std::env::var("CARGO_MANIFEST_DIR").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    );
    let soundpacks_src = manifest_dir.join("soundpacks");
    if !soundpacks_src.is_dir() {
        return Ok(());
    }

    let file = fs::File::create(&zip_path)?;
    let mut zip = zip::ZipWriter::new(file);
    let options = zip::write::SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated);

    fn add_dir(
        zip: &mut zip::ZipWriter<fs::File>,
        base: &Path,
        current: &Path,
        options: zip::write::SimpleFileOptions,
    ) -> io::Result<()> {
        for entry in fs::read_dir(current)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path
                .strip_prefix(base)
                .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
            let relative_str = relative.to_string_lossy().replace('\\', "/");

            if path.is_dir() {
                zip.add_directory(&relative_str, options)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                add_dir(zip, base, &path, options)?;
            } else if path.is_file() {
                zip.start_file(&relative_str, options)
                    .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                let mut f = fs::File::open(&path)?;
                io::copy(&mut f, zip)?;
            }
        }
        Ok(())
    }

    add_dir(&mut zip, &soundpacks_src, &soundpacks_src, options)?;
    zip.finish().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(())
}

fn main() -> io::Result<()> {
    sync_soundpacks();
    package_default_soundpacks()?;

    #[cfg(windows)]
    {
        sync_webview2_loader();

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
