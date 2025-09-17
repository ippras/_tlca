// use egui::{
//     Button, ComboBox, Grid, Id, InnerResponse, PopupCloseBehavior, Response, ScrollArea, Ui,
//     Widget, style::Widgets,
// };
// use egui_phosphor::regular::{MINUS, PLUS};
// use lipid::prelude::*;
// use polars::prelude::*;
// use serde::{Deserialize, Serialize};
// use std::{hash::Hash, num::NonZeroI8};

// /// Fatty acid widget
// pub(crate) struct FattyAcidWidget {
//     fatty_acid: Option<FattyAcidChunked>,
//     id_salt: Id,
//     editable: bool,
//     hover: bool,
// }

// impl FattyAcidWidget {
//     pub(crate) fn new(fatty_acid: Option<FattyAcidChunked>) -> Self {
//         Self {
//             fatty_acid,
//             id_salt: Id::new("FattyAcid"),
//             editable: false,
//             hover: false,
//         }
//     }

//     pub fn id_salt(mut self, id_salt: impl Hash) -> Self {
//         self.id_salt = Id::new(id_salt);
//         self
//     }

//     pub(crate) fn editable(self, editable: bool) -> Self {
//         Self { editable, ..self }
//     }

//     pub(crate) fn hover(self) -> Self {
//         Self {
//             hover: true,
//             ..self
//         }
//     }

//     pub(crate) fn show(self, ui: &mut Ui) -> InnerResponse<Option<FattyAcidChunked>> {
//         let mut inner = None;
//         // None
//         let Some(mut fatty_acid) = self.fatty_acid else {
//             let mut response = ui.label("None");
//             if self.editable {
//                 let mut changed = false;
//                 response.context_menu(|ui| {
//                     if ui.button("Some").clicked() {
//                         inner = Some(FattyAcidChunked::default());
//                         changed = true;
//                         ui.close_menu();
//                     }
//                 });
//                 if changed {
//                     response.mark_changed();
//                 };
//             }
//             return InnerResponse::new(inner, response);
//         };
//         // Some
//         let text = &format!("{:#}", fatty_acid.display(Default::default()));
//         let mut response = if self.editable {
//             let mut changed = false;
//             let mut response = ui.add_sized(
//                 [ui.available_width(), ui.spacing().interact_size.y],
//                 |ui: &mut Ui| {
//                     let response = ui
//                         .menu_button(text, |ui| {
//                             ui.set_max_height(ui.spacing().combo_height);
//                             let response =
//                                 FattyAcidContent::new(self.id_salt, &mut fatty_acid).show(ui);
//                             inner = Some(fatty_acid.clone());
//                             changed |= response.changed();
//                         })
//                         .response;
//                     // Popup::menu(&response).close_behavior(PopupCloseBehavior::IgnoreClicks);
//                     response
//                 },
//             );
//             response.context_menu(|ui| {
//                 let response = ui.button("None");
//                 if response.clicked() {
//                     inner = None;
//                     changed = true;
//                     ui.close_menu();
//                 }
//             });
//             if changed {
//                 response.mark_changed();
//             };
//             response
//         } else {
//             ui.label(text)
//         };
//         if self.hover {
//             response = response.on_hover_text(text);
//         }
//         if let Some(fatty_acid) = inner.clone() {
//             fatty_acid.into_struct(PlSmallStr::EMPTY).unwrap();
//         }
//         InnerResponse::new(inner, response)
//     }
// }

// impl Widget for FattyAcidWidget {
//     fn ui(self, ui: &mut Ui) -> Response {
//         self.show(ui).response
//     }
// }

// /// Fatty acid content
// struct FattyAcidContent<'a> {
//     id_salt: Id,
//     fatty_acid: &'a mut FattyAcidChunked,
// }

// impl<'a> FattyAcidContent<'a> {
//     fn new(id_salt: Id, fatty_acid: &'a mut FattyAcidChunked) -> Self {
//         Self {
//             id_salt,
//             fatty_acid,
//         }
//     }

//     fn show(&mut self, ui: &mut Ui) -> Response {
//         let widgets = if ui.visuals().dark_mode {
//             Widgets::dark()
//         } else {
//             Widgets::light()
//         };
//         ui.visuals_mut().widgets.inactive.weak_bg_fill = widgets.hovered.weak_bg_fill;
//         ui.visuals_mut().widgets.hovered.bg_stroke = widgets.hovered.bg_stroke;

//         let mut change = None;
//         let mut response = ui.response();
//         let height = ui.spacing().interact_size.y;
//         let width = ui.spacing().combo_width / 2.0;
//         ui.vertical_centered_justified(|ui| {
//             ui.label(format!("{:#}", self.fatty_acid.display(Default::default())));
//         });
//         ui.separator();
//         Grid::new(ui.auto_id_with(self.id_salt)).show(ui, |ui| {
//             if ui.add_sized([width, height], Button::new(MINUS)).clicked() {
//                 self.fatty_acid.pop();
//                 response.mark_changed();
//             }
//             if ui.add_sized([width, height], Button::new(PLUS)).clicked() {
//                 self.fatty_acid.push().unwrap();
//                 response.mark_changed();
//             }
//         });
//         ui.separator();
//         ScrollArea::vertical().show(ui, |ui| {
//             Grid::new(ui.auto_id_with(self.id_salt)).show(ui, |ui| {
//                 for (index, (mut offset, mut bound)) in self.fatty_acid.iter().enumerate() {
//                     let text = match offset {
//                         Some(Some(index)) => &index.to_string(),
//                         Some(None) => "?",
//                         None => "*",
//                     };
//                     let delta = (index + 1) as i8;
//                     if let Some(response) = ComboBox::from_id_salt(ui.auto_id_with(self.id_salt))
//                         .close_behavior(PopupCloseBehavior::CloseOnClickOutside)
//                         .width(width)
//                         .selected_text(text)
//                         .show_ui(ui, |ui| {
//                             let mut response = ui.selectable_value(
//                                 &mut offset,
//                                 Some(NonZeroI8::new(delta)),
//                                 delta.to_string(),
//                             );
//                             response |= ui.selectable_value(&mut offset, Some(None), "?");
//                             response |= ui.selectable_value(&mut offset, None, "*");
//                             response
//                         })
//                         .inner
//                         && response.changed()
//                     {
//                         change = Some(Kind::Index(Change {
//                             index,
//                             value: offset,
//                         }));
//                     }
//                     let text = format!("{bound:#}");
//                     if let Some(response) = ComboBox::from_id_salt(ui.auto_id_with(self.id_salt))
//                         .width(width)
//                         .selected_text(text)
//                         .show_ui(ui, |ui| {
//                             let mut response = ui.selectable_value(
//                                 &mut bound,
//                                 Bound::S,
//                                 format!("{:#}", Bound::S),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::DC,
//                                 format!("{:#}", Bound::DC),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::DT,
//                                 format!("{:#}", Bound::DT),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::D,
//                                 format!("{:#}", Bound::D),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::TC,
//                                 format!("{:#}", Bound::TC),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::TT,
//                                 format!("{:#}", Bound::TT),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::T,
//                                 format!("{:#}", Bound::T),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::UC,
//                                 format!("{:#}", Bound::UC),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::UT,
//                                 format!("{:#}", Bound::UT),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::U,
//                                 format!("{:#}", Bound::U),
//                             );
//                             response |= ui.selectable_value(
//                                 &mut bound,
//                                 Bound::B,
//                                 format!("{:#}", Bound::B),
//                             );
//                             response
//                         })
//                         .inner
//                         && response.changed()
//                     {
//                         change = Some(Kind::Bound(Change {
//                             index,
//                             value: bound,
//                         }));
//                     }
//                     ui.end_row();
//                 }
//             });
//         });
//         if let Some(kind) = change {
//             match kind {
//                 Kind::Index(change) => {
//                     let index = Int8Chunked::from_iter_options(
//                         INDEX.into(),
//                         self.fatty_acid
//                             .index()
//                             .iter()
//                             .enumerate()
//                             .map(|(index, value)| {
//                                 if index == change.index {
//                                     Some(match change.value? {
//                                         Some(value) => value.get(),
//                                         None => 0,
//                                     })
//                                 } else {
//                                     value
//                                 }
//                             }),
//                     );
//                     *self.fatty_acid.index_mut() = index;
//                 }
//                 Kind::Bound(change) => {
//                     let bound = self
//                         .fatty_acid
//                         .bound()
//                         .iter()
//                         .enumerate()
//                         .map(|(index, value)| {
//                             if index == change.index {
//                                 change.value
//                             } else {
//                                 value
//                             }
//                         })
//                         .collect::<BoundChunked>();
//                     *self.fatty_acid.bound_mut() = bound;
//                 }
//             }
//             response.mark_changed();
//         }
//         response
//     }
// }

// /// Kind
// #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
// enum Kind {
//     Index(Change<Option<Option<NonZeroI8>>>),
//     Bound(Change<Bound>),
// }

// /// Change
// #[derive(Clone, Copy, Debug, Deserialize, Serialize)]
// struct Change<T> {
//     index: usize,
//     value: T,
// }
