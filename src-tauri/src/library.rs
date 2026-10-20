use std::path::{Path, PathBuf};
use tauri::Manager;
use walkdir::WalkDir;

#[derive(serde::Serialize)]
pub struct ScannedFile {
    pub path: String,
    pub name: String,
    pub size: u64,
}

/// Walk `dir` and return the paths of every .epub file found.
fn find_epubs(dir: &Path, recursive: bool) -> Vec<PathBuf> {
    let max_depth = if recursive { usize::MAX } else { 1 };

    WalkDir::new(dir)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.into_path())
        .filter(|path| {
            path.extension()
                .map(|e| e.eq_ignore_ascii_case("epub"))
                .unwrap_or(false)
        })
        .collect()
}

fn to_scanned_file(path: &Path) -> ScannedFile {
    ScannedFile {
        path: path.to_string_lossy().into_owned(),
        name: path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        size: std::fs::metadata(path).map(|m| m.len()).unwrap_or(0),
    }
}

// TODO: add ability to import and read all types of files instead of only .epub

/// Copy every .epub under `dir` into `library`, skipping books that are
/// already there. Returns how many books were newly imported.
fn import_sync(dir: &Path, recursive: bool, library: &Path) -> Result<usize, String> {
    let mut imported = 0;

    for path in find_epubs(dir, recursive) {
        let Some(file_name) = path.file_name() else {
            continue;
        };
        let dest = library.join(file_name);

        // A book with this file name is already in the library so we don't import it
        if dest.exists() {
            continue;
        }

        // One unreadable file shouldn't abort the whole import.
        if let Err(e) = std::fs::copy(&path, &dest) {
            eprintln!("failed to import {}: {e}", path.display());
            continue;
        }
        imported += 1;
    }
    Ok(imported)
}

fn ensure_library_dir(app: &tauri::AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("books");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir)
}

#[tauri::command]
pub fn library_dir(app: tauri::AppHandle) -> Result<String, String> {
    Ok(ensure_library_dir(&app)?.to_string_lossy().into_owned())
}

/// List the books that are in the app's library directory.
#[tauri::command]
pub async fn scan_epubs(app: tauri::AppHandle) -> Result<Vec<ScannedFile>, String> {
    let library = ensure_library_dir(&app)?;
    tauri::async_runtime::spawn_blocking(move || {
        find_epubs(&library, false)
            .iter()
            .map(|path| to_scanned_file(path))
            .collect()
    })
    .await
    .map_err(|e| format!("join error: {e}"))
}

/// Copy the .epub files found in `dir` into the app's library directory.
#[tauri::command]
pub async fn import_epubs(
    app: tauri::AppHandle,
    dir: String,
    recursive: bool,
) -> Result<usize, String> {
    let library = ensure_library_dir(&app)?;
    tauri::async_runtime::spawn_blocking(move || import_sync(Path::new(&dir), recursive, &library))
        .await
        .map_err(|e| format!("join error: {e}"))?
}
