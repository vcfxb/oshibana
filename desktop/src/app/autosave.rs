use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};
use std::time::Duration;

pub struct AutoSaveState {
    pub autosave: Arc<AtomicBool>,
    pub autosave_thread: JoinHandle<()>,
    pub callback: Arc<dyn Fn() -> anyhow::Result<()> + Send + Sync>
}

impl AutoSaveState {
    pub const INTERVAL: Duration = Duration::from_secs(2);

    pub fn start_new(autosave: bool, cb: impl 'static + Fn() -> anyhow::Result<()> + Send + Sync) -> Self {
        let atomic = Arc::new(AtomicBool::new(autosave));
        let cb = Arc::new(cb);
        
        let atomic_clone = Arc::clone(&atomic);
        let cb_clone = Arc::clone(&cb);
        
        let thread = thread::spawn(move || loop {
            thread::sleep(Self::INTERVAL);

            if atomic_clone.load(Ordering::Acquire) {
                if let Err(err) = cb_clone() {
                    log::error!("error calling autosave function: {err}");
                }
            }
        });

        Self {
            autosave: atomic,
            autosave_thread: thread,
            callback: cb
        }
    }
}