use tauri::Manager;
use walkdir::WalkDir;

#[derive(serde::Serialize)]
pub struct ScannedFile {
    pub path: String,
    pub name: String,
    pub size: u64,
}

fn scan_sync(dir: &str, recursive: bool) -> Result<Vec<ScannedFile>, String> {
    let max_depth = if recursive { usize::MAX } else { 1 };
    let mut files = Vec::new();

    for entry in WalkDir::new(dir)
        .max_depth(max_depth)
        .into_iter()
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let is_epub = path
            .extension()
            .map(|e| e.eq_ignore_ascii_case("epub"))
            .unwrap_or(false);
        if !is_epub {
            continue;
        }
        let size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
        files.push(ScannedFile {
            path: path.to_string_lossy().into_owned(),
            name: path
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            size,
        });
    }
    Ok(files)
}

fn ensure_library_dir(app: &tauri::AppHandle) -> Result<std::path::PathBuf, String> {
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

#[tauri::command]
pub async fn scan_epubs(
    app: tauri::AppHandle,
    dir: Option<String>,
    recursive: bool,
) -> Result<Vec<ScannedFile>, String> {
    // Fall back to the app's library dir when no directory is provided.
    let dir = match dir {
        Some(dir) => dir,
        None => ensure_library_dir(&app)?.to_string_lossy().into_owned(),
    };
    tauri::async_runtime::spawn_blocking(move || scan_sync(&dir, recursive))
        .await
        .map_err(|e| format!("join error: {e}"))?
}
