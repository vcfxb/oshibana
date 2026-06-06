//! We want scryfall sync to render over the whole window so we don't bother with the usual
//! View stuff.

use egui::Ui;
use humansize::{FormatSizeOptions, Kilo, format_size, format_size_i};
use std::sync::atomic::Ordering;
use storage::scryfall::sync_handler::{SyncHandler, SyncState};

/// Draw the loading bar UI that should appear when syncing from scryfall.
pub fn ui(sync_handler: &SyncHandler, ui: &mut Ui) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(ui.available_height() / 3.0);
            ui.heading("Syncing Scryfall Data");
            ui.add_space(20.0);

            let total = sync_handler.sync_size.load(Ordering::Acquire);
            let progress = sync_handler.displayed_bytes.load(Ordering::Acquire);
            let rate = sync_handler.displayed_rate.load(Ordering::Acquire);
            let progress_percent = (progress as f32 / total as f32)
                // clamp because it seems that if it's too close to 0 (e-6), it might crash the
                // ui?
                .clamp(0.01, 1.0);

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
                sync_handler
                    .displayed_cards_transformed
                    .load(Ordering::Acquire)
            ));

            let msg = match *sync_handler.sync_state.try_lock().unwrap() {
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
