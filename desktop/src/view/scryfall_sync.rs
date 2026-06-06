//! We want scryfall sync to render over the whole window so we don't bother with the usual
//! View stuff.

use std::sync::{Arc, LazyLock};
use atomic_float::AtomicF32;
use egui::Ui;
use humansize::{FormatSizeOptions, Kilo, format_size, format_size_i};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use atomic_time::AtomicInstant;
use storage::scryfall::sync_handler::{SyncHandler, SyncState};

pub struct SyncView {
    displayed_cards: AtomicUsize,
    displayed_rate: AtomicF32,
    displayed_bytes_downloaded: AtomicUsize,
}

impl SyncView {
    pub fn new() -> Self {
        SyncView {
            displayed_cards: AtomicUsize::new(0),
            displayed_rate: AtomicF32::new(0.0),
            displayed_bytes_downloaded: AtomicUsize::new(0),
        }
    }

    pub fn make_progress_cb(
        self: Arc<Self>,
        sync_handler: Arc<SyncHandler>
    ) -> impl Fn(usize, Duration) + Send + 'static {
        const DISPLAY_TICK_INTERVAL: Duration = Duration::from_millis(300);
        static LAST_DISPLAY_TICK: LazyLock<AtomicInstant> = LazyLock::new(AtomicInstant::now);

        move |bytes_downloaded: usize, elapsed: Duration| {
            let last_call =
                bytes_downloaded == sync_handler.expected_size.load(Ordering::Acquire);

            let interval_passed =
                LAST_DISPLAY_TICK.load(Ordering::Acquire).elapsed() >= DISPLAY_TICK_INTERVAL;

            if last_call || interval_passed {
                self.displayed_cards.store(
                    sync_handler.card_count.load(Ordering::Acquire),
                    Ordering::Release
                );

                self.displayed_bytes_downloaded.store(bytes_downloaded, Ordering::Release);

                let rate = bytes_downloaded as f32 / elapsed.as_secs_f32();
                self.displayed_rate.store(rate, Ordering::Release);
                LAST_DISPLAY_TICK.store(Instant::now(), Ordering::Release);
            }
        }
    }

    pub fn ui(&self, sync_handler: &SyncHandler, ui: &mut Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(ui.available_height() / 3.0);
                ui.heading("Syncing Scryfall Data");
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
                    "{} cards downloaded",
                    self.displayed_cards.load(Ordering::Acquire)
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
