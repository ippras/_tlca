use crate::utils::Hashed;

use self::{
    data::Data,
    panes::{Pane, behavior::Behavior},
    widgets::PresetsWidget,
    windows::About,
};
use eframe::{APP_KEY, CreationContext, Storage, get_value, set_value};
use egui::{
    Align, Align2, CentralPanel, Color32, Context, FontDefinitions, Frame, Id, LayerId, Layout,
    Order, RichText, ScrollArea, SidePanel, Sides, TextStyle, TopBottomPanel, Visuals, menu::bar,
    warn_if_debug_build,
};
use egui_ext::{DroppedFileExt, HoveredFileExt, LightDarkButton};
use egui_phosphor::{
    Variant, add_to_fonts,
    regular::{
        ARROWS_CLOCKWISE, ARROWS_HORIZONTAL, GRID_FOUR, INFO, PENCIL, PLUS, SIDEBAR_SIMPLE,
        SQUARE_SPLIT_HORIZONTAL, SQUARE_SPLIT_VERTICAL, TABS, TRASH,
    },
};
use egui_tiles::{ContainerKind, Tile, Tree};
use egui_tiles_ext::{TilesExt as _, TreeExt as _, VERTICAL};
use metadata::MetaDataFrame;
use serde::{Deserialize, Serialize};
use std::{borrow::BorrowMut, fmt::Write, io::Cursor, mem::take, str};
use tracing::{error, info, trace};

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
    // Panels
    left_panel: bool,
    // Data
    data: Data,
    // Panes
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
        let app = Self::load(cc).unwrap_or_default();
        app
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
            bar(ui, |ui| {
                ScrollArea::horizontal().show(ui, |ui| {
                    // Left panel
                    ui.toggle_value(
                        &mut self.left_panel,
                        RichText::new(SIDEBAR_SIMPLE).size(ICON_SIZE),
                    )
                    .on_hover_ui(|ui| {
                        ui.label("left_panel");
                    });
                    ui.separator();
                    // Light/Dark
                    ui.light_dark_button(ICON_SIZE);
                    ui.separator();
                    // Reset
                    if ui
                        .button(RichText::new(TRASH).size(ICON_SIZE))
                        .on_hover_text("reset_application")
                        .clicked()
                    {
                        *self = Default::default();
                    }
                    ui.separator();
                    if ui
                        .button(RichText::new(ARROWS_CLOCKWISE).size(ICON_SIZE))
                        .on_hover_text("reset_gui")
                        .clicked()
                    {
                        ui.memory_mut(|memory| {
                            memory.caches = take(memory).caches;
                        });
                    }
                    ui.separator();
                    if ui
                        .button(RichText::new(SQUARE_SPLIT_VERTICAL).size(ICON_SIZE))
                        .on_hover_text("vertical")
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
                        .on_hover_text("horizontal")
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
                        .on_hover_text("grid")
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
                        .on_hover_text("tabs")
                        .clicked()
                    {
                        if let Some(id) = self.tree.root {
                            if let Some(Tile::Container(container)) = self.tree.tiles.get_mut(id) {
                                container.set_kind(ContainerKind::Tabs);
                            }
                        }
                    }
                    ui.separator();
                    // Resizable
                    let mut resizable = true;
                    if ui
                        .button(RichText::new(ARROWS_HORIZONTAL).size(ICON_SIZE))
                        .on_hover_text("resize")
                        .clicked()
                    {
                        let mut panes = self.tree.tiles.panes_mut().peekable();
                        if let Some(pane) = panes.peek() {
                            resizable ^= pane.settings.resizable;
                        }
                        for pane in panes {
                            pane.settings.resizable = resizable;
                        }
                    };
                    // Editable
                    let mut editable = true;
                    if ui
                        .button(RichText::new(PENCIL).size(ICON_SIZE))
                        .on_hover_text("edit")
                        .clicked()
                    {
                        let mut panes = self.tree.tiles.panes_mut().peekable();
                        if let Some(pane) = panes.peek() {
                            editable ^= pane.settings.editable;
                        }
                        for pane in panes {
                            pane.settings.editable = editable;
                        }
                    };
                    ui.separator();
                    // Load
                    ui.add(PresetsWidget::new(&mut self.tree));
                    // Create
                    if ui.button(RichText::new(PLUS).size(ICON_SIZE)).clicked() {
                        self.tree
                            .insert_pane::<VERTICAL>(Pane::new(Default::default()));
                    }
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
    fn unite(&mut self, ctx: &Context) {
        if let Some(frames) =
            ctx.data_mut(|data| data.remove_temp::<Vec<Hashed<MetaDataFrame>>>(Id::new("Unite")))
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
            for dropped in dropped_files {
                trace!(?dropped);
                let bytes = match dropped.bytes() {
                    Ok(bytes) => bytes,
                    Err(error) => {
                        error!(%error);
                        continue;
                    }
                };
                trace!(?bytes);
                let mut reader = Cursor::new(bytes);
                match MetaDataFrame::read_parquet(&mut reader) {
                    Ok(frame) => {
                        trace!(?frame);
                        self.data.add(Hashed::new(frame));
                        // self.tree.insert_pane::<VERTICAL>(Pane::new(frame));
                    }
                    Err(error) => error!(%error),
                };
            }
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
mod widgets;
mod windows;
