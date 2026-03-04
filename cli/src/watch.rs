use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc;
use std::time::Duration;

/// A filesystem event detected by the watcher.
pub struct WatchEvent {
    pub kind: WatchEventKind,
    pub paths: Vec<String>,
}

/// Classification of filesystem change events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WatchEventKind {
    Created,
    Modified,
    Deleted,
    Other,
}

impl std::fmt::Display for WatchEventKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WatchEventKind::Created => write!(f, "CREATED"),
            WatchEventKind::Modified => write!(f, "MODIFIED"),
            WatchEventKind::Deleted => write!(f, "DELETED"),
            WatchEventKind::Other => write!(f, "OTHER"),
        }
    }
}

/// Real-time filesystem watcher using OS-native APIs.
///
/// Uses `notify-rs` which leverages:
/// - ReadDirectoryChangesW on Windows
/// - inotify on Linux
/// - FSEvents on macOS
pub struct FileSystemWatcher {
    _watcher: RecommendedWatcher,
    rx: mpsc::Receiver<Result<notify::Event, notify::Error>>,
}

impl FileSystemWatcher {
    /// Start watching a directory recursively.
    pub fn new(path: &Path) -> Result<Self, notify::Error> {
        let (tx, rx) = mpsc::channel();

        let mut watcher = RecommendedWatcher::new(
            move |res| {
                let _ = tx.send(res);
            },
            Config::default().with_poll_interval(Duration::from_secs(2)),
        )?;

        watcher.watch(path, RecursiveMode::Recursive)?;

        Ok(Self {
            _watcher: watcher,
            rx,
        })
    }

    /// Poll for events, blocking up to `timeout`.
    /// Returns all events received within the timeout period.
    pub fn poll(&self, timeout: Duration) -> Vec<WatchEvent> {
        let mut events = Vec::new();

        // First event: block up to timeout
        match self.rx.recv_timeout(timeout) {
            Ok(result) => {
                if let Ok(event) = result {
                    events.push(Self::convert_event(event));
                }
            }
            Err(_) => return events, // Timeout, no events
        }

        // Drain any additional queued events (non-blocking)
        while let Ok(result) = self.rx.try_recv() {
            if let Ok(event) = result {
                events.push(Self::convert_event(event));
            }
        }

        events
    }

    fn convert_event(event: notify::Event) -> WatchEvent {
        let kind = match event.kind {
            notify::EventKind::Create(_) => WatchEventKind::Created,
            notify::EventKind::Modify(_) => WatchEventKind::Modified,
            notify::EventKind::Remove(_) => WatchEventKind::Deleted,
            _ => WatchEventKind::Other,
        };

        WatchEvent {
            kind,
            paths: event.paths.iter().map(|p| p.display().to_string()).collect(),
        }
    }
}
