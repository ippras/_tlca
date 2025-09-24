use crate::utils::{Hashed, hash_data_frame};

use self::{
    data::Data,
    panes::{Pane, behavior::Behavior},
    widgets::PresetsWidget,
    windows::About,
};
use anyhow::Result;
use eframe::{APP_KEY, CreationContext, Storage, get_value, set_value};
use egui::{
    Align, Align2, CentralPanel, Color32, Context, DroppedFile, FontDefinitions, Frame, Id,
    LayerId, Layout, MenuBar, Order, RichText, ScrollArea, SidePanel, Sides, TextStyle,
    TopBottomPanel, Visuals, warn_if_debug_build,
};
use egui_ext::{DroppedFileExt, HoveredFileExt, LightDarkButton};
use egui_phosphor::{
    Variant, add_to_fonts,
    regular::{
        ARROWS_CLOCKWISE, GRID_FOUR, INFO, SIDEBAR_SIMPLE, SQUARE_SPLIT_HORIZONTAL,
        SQUARE_SPLIT_VERTICAL, TABS, TRASH,
    },
};
use egui_tiles::{ContainerKind, Tile, Tree};
use egui_tiles_ext::{TreeExt as _, VERTICAL};
use lipid::prelude::*;
use metadata::{MetaDataFrame, Metadata};
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, fmt::Write, io::Cursor, mem::take, str, sync::LazyLock};
use tracing::{info, instrument, trace};

/// IEEE 754-2008
const MAX_PRECISION: usize = 16;

pub(super) const ICON_SIZE: f32 = 32.0;

const VALUE_DATA_TYPE: LazyLock<DataType> = LazyLock::new(|| {
    DataType::Struct(vec![
        Field::new(PlSmallStr::from_static("Mean"), DataType::Float64),
        Field::new(
            PlSmallStr::from_static("StandardDeviation"),
            DataType::Float64,
        ),
        Field::new(
            PlSmallStr::from_static("Repetitions"),
            DataType::Array(Box::new(DataType::Float64), 0),
        ),
    ])
});

pub(crate) type HashedMetaDataFrame = MetaDataFrame<Metadata, Hashed<DataFrame>>;

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
    // Panels
    left_panel: bool,
    // Data
    #[serde(skip)]
    data: Data,
    // Panes
    #[serde(skip)]
    tree: Tree<Pane>,
    // Windows
    #[serde(skip)]
    about: About,
}

impl Default for App {
    fn default() -> Self {
        Self {
            left_panel: true,
            data: Default::default(),
            tree: Tree::empty("CentralTree"),
            about: Default::default(),
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
        custom_style(&cc.egui_ctx);

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
    fn panels(&mut self, ctx: &Context) {
        self.top_panel(ctx);
        self.bottom_panel(ctx);
        self.left_panel(ctx);
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
    fn left_panel(&mut self, ctx: &Context) {
        SidePanel::left("LeftPanel")
            .resizable(true)
            .show_animated(ctx, self.left_panel, |ui| {
                self.data.show(ui);
            });
    }

    // Top panel
    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("TopPanel").show(ctx, |ui| {
            MenuBar::new().ui(ui, |ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    // Left panel
                    ui.toggle_value(
                        &mut self.left_panel,
                        RichText::new(SIDEBAR_SIMPLE).size(ICON_SIZE),
                    )
                    .on_hover_ui(|ui| {
                        ui.label("LeftPanel");
                    });
                    ui.separator();
                    // Light/Dark
                    ui.light_dark_button(ICON_SIZE);
                    ui.separator();
                    // Reset
                    if ui
                        .button(RichText::new(TRASH).size(ICON_SIZE))
                        .on_hover_text("ResetApplication")
                        .clicked()
                    {
                        *self = Default::default();
                    }
                    ui.separator();
                    if ui
                        .button(RichText::new(ARROWS_CLOCKWISE).size(ICON_SIZE))
                        .on_hover_text("ResetGui")
                        .clicked()
                    {
                        ui.memory_mut(|memory| {
                            memory.caches = take(memory).caches;
                        });
                    }
                    ui.separator();
                    if ui
                        .button(RichText::new(SQUARE_SPLIT_VERTICAL).size(ICON_SIZE))
                        .on_hover_text("Vertical")
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Vertical);
                            }
                        }
                    }
                    if ui
                        .button(RichText::new(SQUARE_SPLIT_HORIZONTAL).size(ICON_SIZE))
                        .on_hover_text("Horizontal")
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Horizontal);
                            }
                        }
                    }
                    if ui
                        .button(RichText::new(GRID_FOUR).size(ICON_SIZE))
                        .on_hover_text("Grid")
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Grid);
                            }
                        }
                    }
                    if ui
                        .button(RichText::new(TABS).size(ICON_SIZE))
                        .on_hover_text("Tabs")
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Tabs);
                            }
                        }
                    }
                    ui.separator();
                    // Load
                    ui.add(PresetsWidget);
                    ui.separator();
                    // About
                    if ui
                        .button(RichText::new(INFO).size(ICON_SIZE))
                        .on_hover_text("About window")
                        .clicked()
                    {
                        self.about.open ^= true;
                    }
                    ui.separator();
                });
            });
        });
    }
}

// Windows
impl App {
    fn windows(&mut self, ctx: &Context) {
        self.about.window(ctx);
    }
}

// Copy/Paste, Drag&Drop
impl App {
    fn data(&mut self, ctx: &Context) {
        if let Some(frame) =
            ctx.data_mut(|data| data.remove_temp::<HashedMetaDataFrame>(Id::new("Data")))
        {
            self.data.add(frame);
            self.left_panel = true;
        }
    }

    fn unite(&mut self, ctx: &Context) {
        if let Some(frames) =
            ctx.data_mut(|data| data.remove_temp::<Vec<HashedMetaDataFrame>>(Id::new("Unite")))
        {
            self.tree.insert_pane::<VERTICAL>(Pane::new(frames));
        }
    }

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
            for dropped_file in dropped_files {
                let _ = self.parse(dropped_file);
            }
        }
    }

    #[instrument(skip_all, err)]
    fn parse(&mut self, dropped_file: DroppedFile) -> Result<()> {
        const COMPOSITION: LazyLock<SchemaRef> = LazyLock::new(|| {
            Arc::new(Schema::from_iter([
                field!(LABEL[DataType::String]),
                field!(TRIACYLGLYCEROL[data_type!(FATTY_ACID)]),
                Field::new(PlSmallStr::from_static("Value"), VALUE_DATA_TYPE.clone()),
            ]))
        });

        let bytes = dropped_file.bytes()?;
        trace!(?bytes);
        let mut frame = MetaDataFrame::read_parquet(Cursor::new(bytes))?;
        let schema = frame.data.schema();
        if COMPOSITION.matches_schema(schema).is_ok_and(|cast| !cast) {
            info!("COMPOSITION");
            let hash = hash_data_frame(&mut frame.data)?;
            self.data.add(MetaDataFrame {
                meta: frame.meta,
                data: Hashed {
                    value: frame.data,
                    hash,
                },
            });
        } else {
            return Err(
                polars_err!(SchemaMismatch: r#"Invalid dropped file schema: expected [`COMPOSITION`], got = `{schema:?}`"#),
            )?;
        }
        Ok(())
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
        self.data(ctx);
        self.unite(ctx);
        // Pre update
        self.panels(ctx);
        self.windows(ctx);
        // Post update
        self.drag_and_drop(ctx);
    }
}

mod computers;
mod data;
mod panes;
mod parameters;
mod widgets;
mod windows;
