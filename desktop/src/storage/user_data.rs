use std::{fs, thread};
use std::path::PathBuf;
use std::sync::{Arc, LazyLock, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use atomic_float::AtomicF32;
use atomic_time::AtomicInstant;
use crate::storage::DATA_DIR;
use crate::storage::user_data::schema::UserData;

pub mod schema;
pub mod deck;
pub mod collection;
pub mod wishlist;
pub mod package;

pub static USER_DATA_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    DATA_DIR.join("user_data.json")
});

pub struct UserDataStorage {
    pub loaded: RwLock<UserData>,
    has_pending_updates: AtomicBool,
    pub autosave_interval_secs: AtomicF32,
    currently_saving: AtomicBool,
    last_save: AtomicInstant,
}

impl UserDataStorage {
    fn load_from_fs() -> anyhow::Result<UserData> {
        let start = Instant::now();
        log::info!("reading user data from {}", USER_DATA_PATH.display());
        
        let file_content = fs::read(&*USER_DATA_PATH)
            .map_err(|err| {
                log::warn!("failed to read file: {err}");
                err
            })?;

        let data = serde_json::from_slice(file_content.as_slice())
            .map_err(|err| {
                log::warn!("failed to deserialize: {err}");
                err
            })?;

        log::info!("read user data successfully in {:?}", start.elapsed());
        Ok(data)
    }

    pub fn new() -> anyhow::Result<Arc<Self>> {
        if !USER_DATA_PATH.exists() {
            log::info!("no user data file found. creating: {}", USER_DATA_PATH.display());
            
            let bytes = serde_json::to_vec(&UserData::default())
                .expect("default userdata serializes successfully");
            
            fs::write(&*USER_DATA_PATH, bytes)?;
            log::info!("wrote user data file");
        }
        
        let user_data = RwLock::new(Self::load_from_fs()?);
        let interval = AtomicF32::new(2.0);
        let has_pending_changes = AtomicBool::new(false);
        let currently_saving = AtomicBool::new(false);

        let storage = Arc::new(Self {
            loaded: user_data,
            has_pending_updates: has_pending_changes,
            autosave_interval_secs: interval,
            currently_saving,
            last_save: AtomicInstant::now()
        });

        let storage_clone = Arc::clone(&storage);
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_millis(200));
                let last_save = storage_clone.last_save.load(Ordering::Acquire);
                let interval = storage_clone.autosave_interval_secs.load(Ordering::Relaxed);
                if last_save.elapsed().as_secs_f32() < interval {
                    continue;
                }

                if storage_clone.has_pending_updates.load(Ordering::Acquire) {
                    // fail-open on saving -- if the save fails, try again next iteration.
                    let _ = storage_clone.save();
                }
                storage_clone.last_save.store(Instant::now(), Ordering::Release);
            }
        });

        Ok(storage)
    }

    pub fn save(&self) -> anyhow::Result<()> {
        struct SaveGuard<'a> {
            currently_saving: &'a AtomicBool
        }

        impl<'a> Drop for SaveGuard<'a> {
            fn drop(&mut self) {
                self.currently_saving.store(false, Ordering::Release);
            }
        }

        let start = Instant::now();
        self.currently_saving.store(true, Ordering::Release);
        // guard drops and turns of "currently saving" at the end of
        // each iteration.
        let save_guard = SaveGuard {
            currently_saving: &self.currently_saving
        };

        self.has_pending_updates.store(false, Ordering::Release);

        let new_save_path = USER_DATA_PATH.with_added_extension(".new");

        let user_data_read_guard = self
            .loaded
            .read()
            .unwrap();

        let mpk_bytes = serde_json::to_vec(&*user_data_read_guard)
            .map_err(|err| {
                log::error!("failed to serialize userdata: {err}");
                err
            })?;

        drop(user_data_read_guard);

        fs::write(&new_save_path, &mpk_bytes).map_err(|err| {
            log::error!("failed to write userdata to file: {err}");
            err
        })?;


        // assume this rename is atomic
        fs::rename(&new_save_path, &*USER_DATA_PATH).map_err(|err| {
            log::error!("failed to update userdata: {err}");
            err
        })?;

        log::info!("Saved successfully in {:?}", start.elapsed());
        drop(save_guard);
        Ok(())
    }

    pub fn trigger_save(&self) {
        self.has_pending_updates.store(true, Ordering::Release);
    }

    pub fn has_pending_updates(&self) -> bool {
        self.has_pending_updates.load(Ordering::Acquire)
    }

    pub fn currently_saving(&self) -> bool {
        self.currently_saving.load(Ordering::Acquire)
    }
}
