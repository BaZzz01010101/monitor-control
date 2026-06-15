use std::{
    fs::{self, File, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Context;

#[derive(Clone)]
pub struct FileLogger {
    path: PathBuf,
    file: Arc<Mutex<File>>,
}

impl FileLogger {
    pub fn for_current_user() -> anyhow::Result<Self> {
        let appdata = std::env::var_os("APPDATA")
            .map(PathBuf::from)
            .context("APPDATA is not set")?;
        let dir = appdata.join("Dell Controller");
        fs::create_dir_all(&dir)
            .with_context(|| format!("failed to create log directory {}", dir.display()))?;
        let path = dir.join("dell-controller.log");
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .with_context(|| format!("failed to open log file {}", path.display()))?;

        Ok(Self {
            path,
            file: Arc::new(Mutex::new(file)),
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn log(&self, message: impl AsRef<str>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        if let Ok(mut file) = self.file.lock() {
            let _ = writeln!(file, "[{timestamp}] {}", message.as_ref());
        }
    }
}

pub fn log_or_stderr(logger: Option<&FileLogger>, message: impl AsRef<str>) {
    if let Some(logger) = logger {
        logger.log(message);
    } else {
        eprintln!("{}", message.as_ref());
    }
}
