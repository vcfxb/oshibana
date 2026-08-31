//! Indicator bar for when search is processing.

use egui::{Color32, CornerRadius, Pos2, Rect, Ui, pos2, vec2};

pub fn indicator_bar(ui: &mut Ui, search_rect: Rect, active: bool) {
    let height = 3.0;
    let offset = 1.0;

    ui.add_space(height + offset);

    let rect = Rect {
        min: Pos2 {
            y: search_rect.max.y + offset,
            x: search_rect.min.x,
        },
        max: Pos2 {
            y: search_rect.max.y + height + offset,
            x: search_rect.max.x,
        },
    };

    let rounding = CornerRadius {
        ne: 0,
        nw: 0,
        ..ui.visuals().widgets.inactive.corner_radius
    };

    match active {
        false => {
            ui.painter().rect_filled(rect, 0.0, Color32::DARK_GREEN);
        }

        true => {
            ui.ctx().request_repaint();
            let time = ui.time();
            let speed = 4f64;
            let bar_width = rect.width() / 3.0;
            let progress = (time * speed).sin() * 0.5 + 0.5;
            let max_x_offset = rect.width() - bar_width;
            let current_x = rect.left() + (progress * (max_x_offset as f64)) as f32;
            let bar_rect =
                Rect::from_min_size(pos2(current_x, rect.top()), vec2(bar_width, rect.height()));

            ui.painter()
                .rect_filled(rect, rounding, ui.visuals().extreme_bg_color);
            ui.painter()
                .rect_filled(bar_rect, rounding, Color32::LIGHT_BLUE);
        }
    }
}
