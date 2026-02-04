use self::{
    data::Data,
    panes::{Behavior, Pane},
    widgets::{Github, Presets},
};
use crate::{
    app::{
        states::State,
        widgets::{
            about::About,
            buttons::{
                AboutButton, GridButton, HorizontalButton, LeftPanelButton, ReactiveButton,
                ResetButton, TabsButton, VerticalButton,
            },
        },
    },
    r#const::VALUE,
    localization::ContextExt as _,
    utils::HashedMetaDataFrame,
};
use anyhow::Result;
use eframe::{APP_KEY, CreationContext, Storage, get_value, set_value};
use egui::{
    Align, Align2, CentralPanel, Color32, Context, DroppedFile, FontDefinitions, Frame, Id,
    LayerId, Layout, MenuBar, Order, RichText, ScrollArea, SidePanel, Sides, TextStyle,
    TopBottomPanel, Ui, Visuals, Widget as _, Window, warn_if_debug_build,
};
use egui_ext::{DroppedFileExt, HoveredFileExt, LightDarkButton};
use egui_extras::install_image_loaders;
use egui_phosphor::{
    Variant, add_to_fonts,
    regular::{INFO, SLIDERS_HORIZONTAL},
};
use egui_tiles::{Tile, Tree};
use egui_tiles_ext::{TreeExt as _, VERTICAL};
use lipid::prelude::*;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, fmt::Write, str, sync::LazyLock};
use tracing::{error, info, instrument, trace};

const ID_SOURCE: &str = "TLCA";
/// IEEE 754-2008
const MAX_PRECISION: usize = 16;
pub(super) const ICON_SIZE: f32 = 32.0;

fn custom_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = custom_visuals(style.visuals);
    ctx.set_style(style);
}

fn custom_visuals<T: BorrowMut<Visuals>>(mut visuals: T) -> T {
    visuals.borrow_mut().collapsing_header_frame = true;
    visuals
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    // Data
    // #[serde(skip)]
    data: Data,
    // Panes
    #[serde(skip)]
    tree: Tree<Pane>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            data: Default::default(),
            tree: Tree::empty("CentralTree"),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext) -> Self {
        // Customize style of egui.
        let mut fonts = FontDefinitions::default();
        add_to_fonts(&mut fonts, Variant::Regular);
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_localizations();
        custom_style(&cc.egui_ctx);
        install_image_loaders(&cc.egui_ctx);

        // return Default::default();
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        Self::load(cc).unwrap_or_default()
    }

    fn load(cc: &CreationContext) -> Option<Self> {
        let storage = cc.storage?;
        let value = get_value(storage, APP_KEY)?;
        Some(value)
    }
}

// Panels
impl App {
    fn panels(&mut self, ctx: &Context, state: &mut State) {
        self.top_panel(ctx, state);
        self.bottom_panel(ctx);
        self.left_panel(ctx, state);
        self.central_panel(ctx);
    }

    // Bottom panel
    fn bottom_panel(&mut self, ctx: &Context) {
        TopBottomPanel::bottom("BottomPanel").show(ctx, |ui| {
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                Sides::new().show(
                    ui,
                    |_| {},
                    |ui| {
                        warn_if_debug_build(ui);
                        ui.label(RichText::new(env!("CARGO_PKG_VERSION")).small());
                        ui.separator();
                    },
                );
            });
        });
    }

    // Central panel
    fn central_panel(&mut self, ctx: &Context) {
        CentralPanel::default()
            .frame(Frame::central_panel(&ctx.style()).inner_margin(0))
            .show(ctx, |ui| {
                let mut behavior = Behavior { close: None };
                self.tree.ui(&mut behavior, ui);
                if let Some(id) = behavior.close {
                    self.tree.tiles.remove(id);
                }
            });
    }

    // Left panel
    fn left_panel(&mut self, ctx: &Context, state: &mut State) {
        SidePanel::left("LeftPanel").resizable(true).show_animated(
            ctx,
            state.settings.left_panel,
            |ui| {
                self.data.show(ui);
            },
        );
    }

    // Top panel
    fn top_panel(&mut self, ctx: &Context, state: &mut State) {
        TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    // Left panel
                    LeftPanelButton::new(&mut state.settings.left_panel)
                        .size(ICON_SIZE)
                        .ui(ui);
                    ui.separator();
                    ReactiveButton::new(&mut state.settings.reactive)
                        .size(ICON_SIZE)
                        .ui(ui);
                    ui.separator();
                    // Light/Dark
                    ui.light_dark_button(ICON_SIZE);
                    ui.separator();
                    // Reset
                    ResetButton::new(&mut state.settings.reset_state)
                        .size(ICON_SIZE)
                        .ui(ui);
                    ui.separator();
                    // if ui
                    //     .button(RichText::new(SQUARE_SPLIT_VERTICAL).size(ICON_SIZE))
                    //     .on_hover_text("Vertical")
                    //     .clicked()
                    // {
                    //     if let Some(id) = self.tree.root {
                    //         if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                    //             container.set_kind(ContainerKind::Vertical);
                    //         }
                    //     }
                    // }
                    // if ui
                    //     .button(RichText::new(SQUARE_SPLIT_HORIZONTAL).size(ICON_SIZE))
                    //     .on_hover_text("Horizontal")
                    //     .clicked()
                    // {
                    //     if let Some(id) = self.tree.root {
                    //         if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                    //             container.set_kind(ContainerKind::Horizontal);
                    //         }
                    //     }
                    // }
                    // if ui
                    //     .button(RichText::new(GRID_FOUR).size(ICON_SIZE))
                    //     .on_hover_text("Grid")
                    //     .clicked()
                    // {
                    //     if let Some(id) = self.tree.root {
                    //         if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                    //             container.set_kind(ContainerKind::Grid);
                    //         }
                    //     }
                    // }
                    // if ui
                    //     .button(RichText::new(TABS).size(ICON_SIZE))
                    //     .on_hover_text("Tabs")
                    //     .clicked()
                    // {
                    //     if let Some(id) = self.tree.root {
                    //         if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                    //             container.set_kind(ContainerKind::Tabs);
                    //         }
                    //     }
                    // }
                    self.layouts(ui, state);
                    ui.separator();
                    // Presets
                    ui.add(Presets);
                    ui.separator();
                    ui.add(Github);
                    ui.separator();
                    // About
                    AboutButton::new(&mut state.windows.open_about)
                        .size(ICON_SIZE)
                        .ui(ui);
                    ui.separator();
                });
            });
        });
    }

    fn layouts(&mut self, ui: &mut Ui, state: &mut State) {
        VerticalButton::new(&mut state.settings.layout.container_kind)
            .size(ICON_SIZE)
            .ui(ui);
        HorizontalButton::new(&mut state.settings.layout.container_kind)
            .size(ICON_SIZE)
            .ui(ui);
        GridButton::new(&mut state.settings.layout.container_kind)
            .size(ICON_SIZE)
            .ui(ui);
        TabsButton::new(&mut state.settings.layout.container_kind)
            .size(ICON_SIZE)
            .ui(ui);
    }
}

// Windows
impl App {
    fn windows(&mut self, ctx: &Context, state: &mut State) {
        self.about_window(ctx, state);
        self.settings_window(ctx, state);
    }

    fn about_window(&mut self, ctx: &Context, state: &mut State) {
        Window::new(format!("{INFO} About"))
            .open(&mut state.windows.open_about)
            .show(ctx, |ui| About.ui(ui));
    }

    fn settings_window(&mut self, ctx: &Context, state: &mut State) {
        Window::new(format!("{SLIDERS_HORIZONTAL} Settings"))
            .open(&mut state.windows.open_settings)
            .show(ctx, |ui| {
                state.settings.show(ui);
            });
    }
}

// Copy/Paste, Drag&Drop
impl App {
    fn drag_and_drop(&mut self, ctx: &Context) {
        // Preview hovering files
        if let Some(text) = ctx.input(|input| {
            (!input.raw.hovered_files.is_empty()).then(|| {
                let mut text = String::from("Dropping files:");
                for file in &input.raw.hovered_files {
                    write!(text, "\n{}", file.display()).ok();
                }
                text
            })
        }) {
            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
            let screen_rect = ctx.screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }
        // Parse dropped files
        if let Some(dropped_files) = ctx.input(|input| {
            (!input.raw.dropped_files.is_empty()).then_some(input.raw.dropped_files.clone())
        }) {
            info!(?dropped_files);
            let mut frames = Vec::with_capacity(dropped_files.len());
            for dropped_file in dropped_files {
                if let Ok(frame) = self.parse(dropped_file) {
                    frames.push(frame);
                }
            }
            ctx.data_mut(|data| data.insert_temp(Id::new("Data"), frames));
        }
    }

    #[instrument(skip_all, err)]
    fn parse(&mut self, dropped_file: DroppedFile) -> Result<HashedMetaDataFrame> {
        let bytes = dropped_file.bytes()?;
        trace!(?bytes);
        Ok(ron::de::from_bytes::<HashedMetaDataFrame>(&bytes)?)
    }

    fn data(&mut self, ctx: &Context, state: &mut State) {
        const COMPOSITION: LazyLock<SchemaRef> = LazyLock::new(|| {
            Arc::new(Schema::from_iter([
                field!(LABEL[DataType::String]),
                field!(TRIACYLGLYCEROL[data_type!(FATTY_ACID)]),
                Field::new(
                    PlSmallStr::from_static(VALUE),
                    DataType::Array(Box::new(DataType::Float64), 0),
                ),
            ]))
        });

        const CACLULATION: LazyLock<SchemaRef> = LazyLock::new(|| {
            Arc::new(Schema::from_iter([
                Field::new(PlSmallStr::from_static(LABEL), DataType::String),
                field!(FATTY_ACID),
                Field::new(
                    PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS123),
                    DataType::Array(Box::new(DataType::Float64), 0),
                ),
                Field::new(
                    PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS13),
                    DataType::Array(Box::new(DataType::Float64), 0),
                ),
                Field::new(
                    PlSmallStr::from_static(STEREOSPECIFIC_NUMBERS2),
                    DataType::Array(Box::new(DataType::Float64), 0),
                ),
            ]))
        });

        if let Some(frames) =
            ctx.data_mut(|data| data.remove_temp::<Vec<HashedMetaDataFrame>>(Id::new("Data")))
        {
            for frame in frames {
                let schema = frame.data.schema();
                if COMPOSITION.matches_schema(schema).is_ok_and(|cast| !cast) {
                    info!("COMPOSITION");
                    self.data.triacylglycerols.add(frame);
                } else if CACLULATION.matches_schema(schema).is_ok_and(|cast| !cast) {
                    info!("CACLULATION");
                    self.data.fatty_acids.add(frame);
                } else {
                    error!(
                        "{}",
                        polars_err!(SchemaMismatch: r#"Invalid dropped file schema: expected [`CACLULATION`, `COMPOSITION`], got = `{schema:?}`"#)
                    );
                }
            }
            state.settings.left_panel = true;
        }
    }

    fn join(&mut self, ctx: &Context) {
        if let Some(frames) = ctx.data_mut(|data| {
            data.remove_temp::<Vec<HashedMetaDataFrame>>(Id::new("Join").with("FattyAcids"))
        }) {
            self.tree.insert_pane::<VERTICAL>(Pane::fatty_acids(frames));
        } else if let Some(frames) = ctx.data_mut(|data| {
            data.remove_temp::<Vec<HashedMetaDataFrame>>(Id::new("Join").with("Triacylglycerols"))
        }) {
            self.tree
                .insert_pane::<VERTICAL>(Pane::triacylglycerols(frames));
        }
    }

    fn state(&mut self, ctx: &Context, state: &mut State) {
        if state.settings.reset_state {
            *self = Default::default();
            // Cache
            let caches = ctx.memory_mut(|memory| memory.caches.clone());
            ctx.memory_mut(|memory| {
                memory.caches = caches;
            });
            ctx.set_localizations();
            state.settings.reset_state = false;
        }
        if let Some(container_kind) = state.settings.layout.container_kind.take()
            && let Some(id) = self.tree.root
            && let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id)
        {
            container.set_kind(container_kind);
        }
        if state.settings.reactive {
            ctx.request_repaint();
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        let mut state = State::load(ctx, Id::new(ID_SOURCE));
        self.data(ctx, &mut state);
        self.join(ctx);
        // Pre update
        self.panels(ctx, &mut state);
        self.windows(ctx, &mut state);
        // Post update
        self.drag_and_drop(ctx);
        self.state(ctx, &mut state);
        state.store(ctx, Id::new(ID_SOURCE));
    }
}

mod computers;
mod data;
mod panes;
mod states;
mod widgets;
