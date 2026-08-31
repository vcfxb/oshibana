//! Card Search view

pub mod col_format;
pub mod indicator_bar;

use crate::app::Oshibana;
use crate::view::{logic_noop, View};
use eframe::Frame;
use egui::text::LayoutJob;
use egui::{Align, Color32, ComboBox, FontFamily, FontId, Layout, ScrollArea, Stroke, TextEdit, TextFormat, Ui};
use egui_extras::{Column, TableBuilder};
use heck::ToPascalCase;
use schemas::oshibana::{SearchViewColumn, UniqueBy};
use std::borrow::Cow;
use std::sync::Arc;
use std::sync::atomic::Ordering;
use storage::scryfall::search::query_parser::Diagnostic;
use strum::IntoEnumIterator;
use storage::scryfall::ScryfallStorage;
use storage::scryfall::search::{Query, SearchHandler};

pub fn search(scryfall_storage: Arc<ScryfallStorage>) -> View {
    View {
        ui: search_ui,
        logic: logic_noop,
        menu: search_menu,
        state: Box::new(SearchState::new(scryfall_storage)),
    }
}

#[derive(Debug, Default)]
pub enum SearchLayout {
    #[default]
    Rows,
    Tiles,

    /// Card images next to their info/oracle_text/etc
    Detailed,
}

pub struct SearchState {
    search_text: String,
    layout: SearchLayout,
    enable_global_search_prefix: bool,
    unique_by: UniqueBy,
    handler: SearchHandler,
}

impl SearchState {
    pub fn new(storage: Arc<ScryfallStorage>) -> Self {
        SearchState {
            search_text: "".to_string(),
            layout: Default::default(),
            enable_global_search_prefix: true,
            unique_by: Default::default(),
            handler: SearchHandler::new(storage),
        }
    }
}

fn search_menu(app: &mut Oshibana, ui: &mut Ui, _: &mut Frame) {
    let search_state: &mut SearchState = app.current_view.state.downcast_mut().unwrap();

    ui.menu_button("Search", |ui| {
        ui.menu_button("Layout", |ui| {
            if ui.button("Rows").clicked() {
                search_state.layout = SearchLayout::Rows;
            }

            if ui.button("Tiles").clicked() {
                search_state.layout = SearchLayout::Tiles;
            }

            if ui.button("Detailed").clicked() {
                search_state.layout = SearchLayout::Detailed;
            }
        });

        let mut user_data_lock = app.user_data_storage.loaded.lock().unwrap();

        // If they have no search columns visible, make at least card name visible so that we
        // don't error out later on lol
        if user_data_lock.visible_search_columns.is_empty() {
            user_data_lock
                .visible_search_columns
                .push(SearchViewColumn::Name);
            app.user_data_storage.mark_pending();
        }

        ui.menu_button("Columns", |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                for col in SearchViewColumn::iter() {
                    let selected = user_data_lock.visible_search_columns.contains(&col);
                    let text = col.into_str().to_pascal_case();
                    if ui.selectable_label(selected, text).clicked() {
                        if !selected {
                            user_data_lock.visible_search_columns.push(col);
                        } else {
                            user_data_lock.visible_search_columns.retain(|s| s != &col);
                        }
                        app.user_data_storage.mark_pending();
                    }
                }
            })
        })
    });
}

fn search_ui(app: &mut Oshibana, ui: &mut Ui, _: &mut Frame) {
    let search_state: &mut SearchState = app.current_view.state.downcast_mut().unwrap();
    let user_data_guard = app.user_data_storage.loaded.lock().unwrap();
    let search_prefix = user_data_guard.search_prefix.as_str();

    ui.horizontal_top(|ui| {
        let mut prefix_layout = LayoutJob::default();

        prefix_layout.append("Enable search prefix: ", 0.0, TextFormat::default());
        prefix_layout.append(
            search_prefix,
            0.0,
            TextFormat {
                font_id: FontId::new(14.0, FontFamily::Monospace),
                ..TextFormat::default()
            },
        );

        if !search_prefix.is_empty() {
            ui.checkbox(&mut search_state.enable_global_search_prefix, prefix_layout);
        }

        ui.with_layout(Layout::right_to_left(Align::TOP), |ui| {
            ComboBox::from_label("Unique By")
                .selected_text(format!("{}", search_state.unique_by))
                .show_ui(ui, |ui| {
                    for option in UniqueBy::iter() {
                        ui.selectable_value(&mut search_state.unique_by, option, option.to_string());
                    }
                });
        });
    });

    ui.horizontal_top(|ui| {
        ui.vertical_centered(|ui| {
            let search_bar = TextEdit::singleline(&mut search_state.search_text)
                .desired_width(f32::INFINITY)
                .font(FontId::monospace(14.0));
            let search_rect = ui.add(search_bar).rect;

            indicator_bar::indicator_bar(
                ui,
                search_rect,
                search_state.handler.busy.load(Ordering::Relaxed)
            );

            ui.separator();
        })
    });

    let visible_cols = &user_data_guard.visible_search_columns;
    let formatting_fns = visible_cols
        .iter()
        .map(|col| col_format::col_format(&app.scryfall_storage, *col))
        .collect::<Vec<_>>();

    let query = match (search_state.enable_global_search_prefix, search_prefix) {
        (false, _) | (_, "") => Cow::Borrowed(search_state.search_text.as_str()),
        (_, prefix) => Cow::Owned(format!("{prefix} {}", search_state.search_text)),
    };

    search_state.handler.search(Query {
        query: Arc::new(query.to_string()),
        cols: user_data_guard.visible_search_columns.clone(),
        unique_by: search_state.unique_by,
    });

    drop(user_data_guard);
    let search_result_guard = search_state.handler.result.lock().unwrap();

    if let Err(err) = &search_result_guard.result {
        ui.scope(|ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Monospace);
            ui.label(format!("search error: {err}"));
        });
    };

    if !search_result_guard.diagnostics.is_empty() {
        for diagnostic in &search_result_guard.diagnostics {
            let stroke = match diagnostic {
                Diagnostic::Error { .. } => Stroke::new(2.0, Color32::RED),
                Diagnostic::Warning { .. } => Stroke::new(1.0, Color32::YELLOW),
            };

            egui::Frame::NONE
                .corner_radius(2)
                .stroke(stroke)
                .outer_margin(10)
                .inner_margin(5)
                .show(ui, |ui| {
                    ui.add_sized([ui.available_width(), 30.0], |ui: &mut Ui| {
                        ui.vertical(|ui| {
                            match diagnostic {
                                Diagnostic::Error { message, fragment: None } => {
                                    ui.label(message);
                                }

                                Diagnostic::Warning { message, fragment } |
                                Diagnostic::Error { message, fragment: Some(fragment) } => {
                                    ui.label(message);
                                    ui.separator();
                                    let mut lj = LayoutJob::default();
                                    let start = fragment.byte_range.start;
                                    let end = fragment.byte_range.end;
                                    let lhs = &fragment.full_query[..start];
                                    let highlight = fragment.as_str();
                                    let rhs = &fragment.full_query[end..];
                                    let monospace_text_format = TextFormat {
                                        font_id: FontId::monospace(14.0),
                                        ..TextFormat::default()
                                    };

                                    let mut highlight_format = monospace_text_format.clone();
                                    highlight_format.background = Color32::LIGHT_YELLOW;
                                    highlight_format.color = Color32::BLACK;

                                    lj.append(lhs, 0.0, monospace_text_format.clone());
                                    lj.append(highlight, 0.0, highlight_format);
                                    lj.append(rhs, 0.0, monospace_text_format);
                                    ui.label(lj);
                                }
                            }
                        }).response
                    });
                });
        }
    }

    let Ok(df) = &search_result_guard.result else {
        return
    };

    let col_count = df
        .columns()
        .iter()
        .filter(|c| !c.name().starts_with("_"))
        .count();

    TableBuilder::new(ui)
        .columns(Column::remainder(), col_count)
        .resizable(true)
        .striped(true)
        .header(16.0, |mut row| {
            let col_name_iter = df
                .get_column_names()
                .into_iter()
                .filter(|s| !s.starts_with("_"));

            for search_col in col_name_iter {
                row.col(|ui| {
                    ui.label(search_col.as_str());
                });
            }
        })
        .body(|body| {
            body.rows(14.0, df.height(), |mut row| {
                // this is potentially slow (polars discourages row indexing),
                // but I don't know of a better way -- table's don't support adding col by col.
                let df_row = df
                    .get_row(row.index())
                    .expect("dataframe index is in bounds");

                let mut values = df_row.0;
                let normal_image_uri = values.pop().unwrap();

                let hover = |ui: &mut Ui| {
                    if let Some(uri) = normal_image_uri.extract_str() {
                        ui.image(uri);
                    }
                };

                for (col_idx, col) in values.iter().enumerate() {
                    row.col(|ui| {
                        let response = formatting_fns[col_idx](col, ui);
                        response.on_hover_ui_at_pointer(hover);
                    });
                }

                // row.response().on_hover_ui_at_pointer(hover);
            })
        });
}
