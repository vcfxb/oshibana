//! We want scryfall sync to render over the whole window so we don't bother with the usual
//! View stuff.

use atomic_float::AtomicF32;
use egui::Ui;
use humansize::{FormatSizeOptions, Kilo, format_size, format_size_i};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use storage::scryfall::callback_reader::ProgressCallback;
use storage::scryfall::sync_handler::{SyncHandler, SyncState};

pub struct SyncView {
    displayed_records: AtomicUsize,
    displayed_rate: AtomicF32,
    displayed_bytes_downloaded: AtomicUsize,
}

impl SyncView {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        SyncView {
            displayed_records: AtomicUsize::new(0),
            displayed_rate: AtomicF32::new(0.0),
            displayed_bytes_downloaded: AtomicUsize::new(0),
        }
    }

    pub fn make_progress_cb(
        self: Arc<Self>,
        sync_handler: Arc<SyncHandler>,
    ) -> impl ProgressCallback + Clone + Send + 'static {
        const DISPLAY_TICK_INTERVAL: Duration = Duration::from_millis(300);

        #[derive(Clone)]
        struct CallbackHandler {
            last_display_tick: Instant,
            sync_view: Arc<SyncView>,
            sync_handler: Arc<SyncHandler>,
        }

        impl ProgressCallback for CallbackHandler {
            fn call(&mut self, bytes_read: usize, elapsed: Duration) {
                let last_call =
                    bytes_read == self.sync_handler.expected_size.load(Ordering::Acquire);

                let interval_passed = self.last_display_tick.elapsed() >= DISPLAY_TICK_INTERVAL;

                if last_call || interval_passed {
                    self.sync_view.displayed_records.store(
                        self.sync_handler.card_count.load(Ordering::Acquire),
                        Ordering::Release,
                    );

                    self.sync_view
                        .displayed_bytes_downloaded
                        .store(bytes_read, Ordering::Release);

                    // force the denominator to never be 0 so that we don't get INFINITY issues
                    let rate = bytes_read as f32 / elapsed.as_secs_f32().max(0.1);
                    self.sync_view.displayed_rate.store(rate, Ordering::Release);
                    self.last_display_tick = Instant::now();
                }
            }
        }

        CallbackHandler {
            last_display_tick: Instant::now(),
            sync_view: self,
            sync_handler,
        }
    }

    pub fn ui(&self, sync_handler: &SyncHandler, ui: &mut Ui) {
        egui::CentralPanel::default().show(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                let name_guard = sync_handler.sync_target.lock().unwrap();
                ui.heading(format!("Syncing Scryfall Data: {name_guard}"));
                drop(name_guard);
                ui.add_space(20.0);

                let total = sync_handler.expected_size.load(Ordering::Acquire);
                let progress = self.displayed_bytes_downloaded.load(Ordering::Acquire);
                let rate = self.displayed_rate.load(Ordering::Acquire);
                let progress_percent = progress as f32 / total as f32;

                let progress_bar = egui::ProgressBar::new(progress_percent)
                    .show_percentage()
                    .desired_width(ui.available_width() - 40.0);

                ui.add(progress_bar);

                let format_options = FormatSizeOptions::default()
                    .decimal_places(2)
                    .kilo(Kilo::Decimal);

                ui.label(format!(
                    "{}/{} ({})",
                    format_size(progress, format_options),
                    format_size(total, format_options),
                    // if rate is ever infinity, ui freezes.
                    // sync handler should check this
                    format_size_i(rate, format_options.suffix("/s"))
                ));

                ui.label(format!(
                    "{} records downloaded",
                    self.displayed_records.load(Ordering::Acquire)
                ));

                let msg = match *sync_handler.sync_state.lock().unwrap() {
                    SyncState::Downloading => "Downloading",
                    SyncState::FsWrite => "Writing card data file",
                    SyncState::Idle => "Done!",
                };

                ui.label(msg);
                ui.add_space(10.0);
                ui.spinner();
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use humansize::FormatSizeOptions;

    #[test]
    #[ignore = "blocks the thread forever"]
    fn check_humansize_behaviour() {
        println!(
            "{}",
            humansize::format_size_i(f32::INFINITY, FormatSizeOptions::default())
        );
    }
}
