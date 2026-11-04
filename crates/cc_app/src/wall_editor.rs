//! Wall-specific properties dialog and compound layer editor.
//!
//! Values are edited in a draft. The drawing changes only after Save, while
//! the plan preview follows each draft change. Cancel discards the draft.

use crate::actions;
use crate::dialog::Dialog;
use crate::i18n::tr;
use crate::state::AppState;
use crate::t;
use crate::theme::{self, PALETTE};
use cc_core::{ElementId, ElementType, Layer, LayerFunction, LocationLine, TypeData, Wall, WallFunction, WallTop};
use cc_engine::Ref;
use egui::{Color32, Context, DragValue, Pos2, RichText, Stroke, Ui, vec2};

pub struct WallEditor {
    id: Option<ElementId>,
    ty: ElementType,
    wall: Wall,
    pub(crate) layers_open: bool,
    layers_before: Vec<Layer>,
}

pub fn open(state: &mut AppState, id: ElementId) {
    let Some(element) = state.project().element(id) else { return };
    let Some(wall) = element.as_wall().cloned() else { return };
    let Some(ty) = state.project().element_type(element.type_id).cloned() else { return };
    state.selection = std::iter::once(id).collect();
    state.dialogs.wall_editor =
        Some(WallEditor { id: Some(id), ty, wall, layers_open: false, layers_before: Vec::new() });
}

/// Edit the active wall type and the defaults for walls not yet drawn.
pub fn open_type(state: &mut AppState, type_id: cc_core::ElementTypeId) {
    let Some(ty) = state.project().element_type(type_id).filter(|ty| ty.category() == cc_core::Category::Wall).cloned()
    else {
        return;
    };
    let wall = Wall {
        height: state.wall_height,
        top: state.wall_top_level.map_or(WallTop::Unconnected, |level| WallTop::Level { level, offset: 0.0 }),
        location_line: state.wall_location_line,
        ..Default::default()
    };
    state.dialogs.wall_editor = Some(WallEditor { id: None, ty, wall, layers_open: false, layers_before: Vec::new() });
}

pub fn show(ctx: &Context, state: &mut AppState) {
    let Some(mut editor) = state.dialogs.wall_editor.take() else { return };
    if editor.id.is_some_and(|id| state.project().element(id).and_then(|e| e.as_wall()).is_none()) {
        return;
    }
    let width = (ctx.content_rect().width() - 90.0).clamp(520.0, 780.0);
    let layers_were_open = editor.layers_open;
    let mut save = 0;
    let mut save_new = 0;
    let response = Dialog::new("wall-properties", t!("wall-editor-title")).width(width).show(
        ctx,
        |ui| main_body(ui, state, &mut editor),
        |f| {
            save = f.primary(t!("action-save"));
            f.secondary(t!("action-cancel"));
            save_new = f.extra(t!("wall-editor-save-new"));
        },
    );
    // Open on the next frame so the click on the main button cannot also
    // activate a control in the newly created modal.
    if layers_were_open {
        show_layers(ctx, state, &mut editor);
    }
    let close = match response.clicked {
        Some(i) if i == save => commit(state, &editor, false),
        Some(i) if i == save_new => commit(state, &editor, true),
        Some(_) => true,
        None if response.dismissed && !editor.layers_open => true,
        None => false,
    };
    if !close {
        state.dialogs.wall_editor = Some(editor);
    }
}

fn main_body(ui: &mut Ui, state: &AppState, editor: &mut WallEditor) {
    let subtitle = editor.id.map_or_else(
        || state.project().type_name(editor.ty.id).to_string(),
        |id| format!("{} · {id}", state.project().type_name(editor.ty.id)),
    );
    ui.label(theme::muted(subtitle).size(12.0));
    ui.add_space(8.0);
    let max_height = (ui.ctx().content_rect().height() - 170.0).clamp(330.0, 620.0);
    egui::ScrollArea::vertical().max_height(max_height).auto_shrink([false, true]).show(ui, |ui| {
        let left_width = (ui.available_width() * 0.54).max(250.0);
        ui.columns(2, |columns| {
            columns[0].set_width(left_width);
            let left = &mut columns[0];
            left.label(RichText::new(t!("type-type")).strong());
            left.text_edit_singleline(&mut editor.ty.name);
            left.add_space(12.0);
            left.label(RichText::new(t!("section-constraints")).strong());
            if matches!(editor.wall.top, WallTop::Unconnected) {
                value_row(left, &t!("prop-height"), &mut editor.wall.height, 0.1..=100.0);
            } else {
                left.label(theme::muted(t!("prop-top-constraint")).size(11.0));
            }
            value_row(left, &t!("prop-base-offset"), &mut editor.wall.base_offset, -100.0..=100.0);
            left.horizontal(|ui| {
                ui.label(t!("prop-location-line"));
                egui::ComboBox::from_id_salt("wall-editor-location")
                    .selected_text(tr(editor.wall.location_line.key()))
                    .show_ui(ui, |ui| {
                        for line in LocationLine::ALL {
                            ui.selectable_value(&mut editor.wall.location_line, line, tr(line.key()));
                        }
                    });
            });
            left.checkbox(&mut editor.wall.join_start, t!("prop-join-start"));
            left.checkbox(&mut editor.wall.join_end, t!("prop-join-end"));
            left.checkbox(&mut editor.wall.structural, t!("prop-structural"));
            left.checkbox(&mut editor.wall.room_bounding, t!("prop-room-bounding"));
            left.add_space(12.0);
            left.horizontal(|ui| {
                ui.label(RichText::new(t!("type-total-thickness")).strong());
                ui.label(cc_views::format::length(editor.ty.thickness()));
            });
            if left.button(t!("wall-editor-layers")).clicked() {
                editor.layers_before = editor.ty.layers().to_vec();
                editor.layers_open = true;
            }
            left.add_space(8.0);
            left.label(RichText::new(t!("comp-preview")).strong());
            paint_preview(left, state, editor.ty.layers(), 145.0);

            let right = &mut columns[1];
            right.label(RichText::new(t!("section-construction")).strong());
            if let TypeData::Wall { function, layers } = &mut editor.ty.data {
                right.horizontal(|ui| {
                    ui.label(t!("prop-function"));
                    egui::ComboBox::from_id_salt("wall-editor-function").selected_text(tr(function.key())).show_ui(
                        ui,
                        |ui| {
                            for choice in WallFunction::ALL {
                                ui.selectable_value(function, choice, tr(choice.key()));
                            }
                        },
                    );
                });
                right.add_space(10.0);
                right.label(RichText::new(t!("section-materials")).strong());
                for (label, layer) in
                    [(t!("wall-editor-exterior"), layers.first()), (t!("wall-editor-interior"), layers.last())]
                {
                    if let Some(layer) = layer {
                        right.horizontal(|ui| {
                            ui.label(label);
                            ui.label(RichText::new(&layer.material).color(PALETTE.text_muted));
                        });
                    }
                }
                right.add_space(12.0);
                right.label(theme::muted(t!("wall-editor-shared")).size(11.0));
            }
        });
    });
}

fn value_row(ui: &mut Ui, label: &str, value: &mut f64, range: std::ops::RangeInclusive<f64>) {
    ui.horizontal(|ui| {
        ui.label(label);
        ui.add(DragValue::new(value).speed(0.01).range(range).suffix(" m"));
    });
}

fn show_layers(ctx: &Context, state: &AppState, editor: &mut WallEditor) {
    let mut ok = 0;
    let width = (ctx.content_rect().width() - 80.0).clamp(500.0, 850.0);
    let response = Dialog::new("wall-layers", t!("wall-editor-layers")).width(width).show(
        ctx,
        |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(t!("type-total-thickness")).strong());
                ui.label(cc_views::format::length(editor.ty.thickness()));
            });
            ui.add_space(8.0);
            let materials: Vec<_> = state.project().materials.iter().map(|m| m.name.clone()).collect();
            let TypeData::Wall { layers, .. } = &mut editor.ty.data else { return };
            let mut remove = None;
            let mut move_to = None;
            egui::ScrollArea::horizontal().show(ui, |ui| {
                egui::Grid::new("wall-editor-layer-grid").num_columns(5).striped(true).spacing([8.0, 6.0]).show(
                    ui,
                    |ui| {
                        for header in ["", "col-function", "prop-material", "prop-thickness", ""] {
                            ui.label(theme::muted(if header.is_empty() { String::new() } else { tr(header) }));
                        }
                        ui.end_row();
                        let count = layers.len();
                        for (i, layer) in layers.iter_mut().enumerate() {
                            ui.label((i + 1).to_string());
                            egui::ComboBox::from_id_salt(("wall-layer-function", i))
                                .width(110.0)
                                .selected_text(tr(layer.function.key()))
                                .show_ui(ui, |ui| {
                                    for function in LayerFunction::ALL {
                                        ui.selectable_value(&mut layer.function, function, tr(function.key()));
                                    }
                                });
                            egui::ComboBox::from_id_salt(("wall-layer-material", i))
                                .width(150.0)
                                .selected_text(&layer.material)
                                .show_ui(ui, |ui| {
                                    for material in &materials {
                                        ui.selectable_value(&mut layer.material, material.clone(), material);
                                    }
                                });
                            ui.add(DragValue::new(&mut layer.thickness).speed(0.002).range(0.001..=2.0).suffix(" m"));
                            ui.horizontal(|ui| {
                                if ui.add_enabled(i > 0, egui::Button::new("↑")).clicked() {
                                    move_to = Some((i, i - 1));
                                }
                                if ui.add_enabled(i + 1 < count, egui::Button::new("↓")).clicked() {
                                    move_to = Some((i, i + 1));
                                }
                                if ui.add_enabled(count > 1, egui::Button::new("×")).clicked() {
                                    remove = Some(i);
                                }
                            });
                            ui.end_row();
                        }
                    },
                );
            });
            if let Some((a, b)) = move_to {
                layers.swap(a, b);
            }
            if let Some(i) = remove {
                layers.remove(i);
            }
            if ui.button(t!("type-add-layer")).clicked() {
                let material = materials.first().cloned().unwrap_or_default();
                layers.push(Layer { material, thickness: 0.02, function: LayerFunction::Finish });
            }
            ui.add_space(10.0);
            paint_preview(ui, state, layers, 125.0);
        },
        |f| {
            ok = f.primary(t!("action-ok"));
            f.secondary(t!("action-cancel"));
        },
    );
    if response.clicked == Some(ok) {
        editor.layers_open = false;
        editor.layers_before.clear();
    } else if response.clicked.is_some() || response.dismissed {
        if let TypeData::Wall { layers, .. } = &mut editor.ty.data {
            *layers = std::mem::take(&mut editor.layers_before);
        }
        editor.layers_open = false;
    }
}

pub(crate) fn paint_preview(ui: &mut Ui, state: &AppState, layers: &[Layer], height: f32) {
    let width = ui.available_width().max(180.0);
    let (rect, _) = ui.allocate_exact_size(vec2(width, height), egui::Sense::hover());
    let p = ui.painter_at(rect);
    p.rect_filled(rect, 2.0, PALETTE.surface);
    p.rect_stroke(rect, 2.0, Stroke::new(1.0, PALETTE.border), egui::StrokeKind::Inside);
    let total: f64 = layers.iter().map(|l| l.thickness.max(0.0)).sum();
    if total <= 0.0 {
        return;
    }
    let area = rect.shrink2(vec2(18.0, 32.0));
    let mut y = area.top();
    for layer in layers {
        let h = area.height() * (layer.thickness.max(0.0) / total) as f32;
        let r = egui::Rect::from_min_max(Pos2::new(area.left(), y), Pos2::new(area.right(), y + h));
        let color = state
            .project()
            .material(&layer.material)
            .map(|m| {
                let [r, g, b] = m.shading_color();
                Color32::from_rgb(r, g, b)
            })
            .unwrap_or(PALETTE.text_muted);
        p.rect_filled(r, 0.0, color);
        p.rect_stroke(r, 0.0, Stroke::new(1.0, PALETTE.text_muted), egui::StrokeKind::Inside);
        // The cut pattern remains legible even if a material's 3D color is dark.
        let mut hatch = area.left() + 9.0;
        while hatch < area.right() {
            p.line_segment(
                [Pos2::new(hatch, y), Pos2::new((hatch - h).max(area.left()), y + h)],
                Stroke::new(0.7, Color32::from_black_alpha(75)),
            );
            hatch += 12.0;
        }
        y += h;
    }
    p.text(
        rect.center_top() + vec2(0.0, 7.0),
        egui::Align2::CENTER_TOP,
        t!("wall-editor-exterior"),
        egui::FontId::proportional(12.0),
        PALETTE.text,
    );
    p.text(
        rect.right_top() + vec2(-8.0, 7.0),
        egui::Align2::RIGHT_TOP,
        cc_views::format::length(total),
        egui::FontId::proportional(11.0),
        PALETTE.text_muted,
    );
    p.text(
        rect.center_bottom() - vec2(0.0, 7.0),
        egui::Align2::CENTER_BOTTOM,
        t!("wall-editor-interior"),
        egui::FontId::proportional(10.0),
        PALETTE.text_muted,
    );
}

fn commit(state: &mut AppState, editor: &WallEditor, new_type: bool) -> bool {
    let name = editor.ty.name.trim();
    if name.is_empty() {
        state.message = t!("wall-editor-name-required");
        return false;
    }
    if editor.ty.layers().is_empty()
        || editor.ty.layers().iter().any(|l| !l.thickness.is_finite() || l.thickness <= 0.0)
    {
        state.message = t!("wall-editor-invalid-layers");
        return false;
    }
    let start = state.session.doc.undo_depth();
    let old_ty = state.project().element_type(editor.ty.id).cloned();
    if editor.id.is_some_and(|id| state.project().element(id).is_none()) {
        return true;
    }
    let Some(old_ty) = old_ty else { return true };
    let mut target = editor.ty.id;
    if new_type {
        let name = if name == old_ty.name { t!("copy-of", "name" => old_ty.name) } else { name.to_string() };
        let command = cc_engine::commands::types::TypeDuplicate {
            type_ref: Ref::Id(old_ty.id.0),
            name: Some(name),
            elements: editor.id.map_or_else(Vec::new, |id| vec![id.0]),
        };
        let Some(id) = actions::run_command(state, command).and_then(|v| v["id"].as_u64()) else { return false };
        target = cc_core::ElementTypeId(id);
    }
    let draft_ty = editor.ty.clone();
    let needs_type_update = draft_ty.data != old_ty.data || (!new_type && draft_ty.name != old_ty.name);
    if needs_type_update
        && !crate::engine_edit::element_type(state, target, None, |ty| {
            if !new_type {
                ty.name = draft_ty.name;
            }
            ty.data = draft_ty.data;
        })
    {
        state.session.undo_to(start);
        return false;
    }
    if editor.id.is_none() {
        if new_type {
            state.tool = state.tool.with_type(target);
        }
        state.wall_height = editor.wall.height;
        state.wall_top_level = match editor.wall.top {
            WallTop::Unconnected => None,
            WallTop::Level { level, .. } => Some(level),
        };
        state.wall_location_line = editor.wall.location_line;
        state.session.doc.break_merge();
        return true;
    }
    let Some(id) = editor.id else { return true };
    let draft_wall = editor.wall.clone();
    // TypeDuplicate already reassigns the wall; compare against the current
    // element so an unchanged instance is not mistaken for a failed edit.
    let changed = state
        .project()
        .element(id)
        .is_some_and(|current| current.as_wall() != Some(&draft_wall) || current.type_id != target);
    if changed
        && !crate::engine_edit::element(state, id, None, |el| {
            el.type_id = target;
            el.kind = cc_core::ElementKind::Wall(draft_wall);
        })
    {
        state.session.undo_to(start);
        return false;
    }
    state.session.doc.break_merge();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> (AppState, ElementId) {
        let project = crate::home::sample_project("casa_simples").unwrap();
        let id = project.elements.values().find(|e| e.as_wall().is_some()).unwrap().id;
        (AppState::new(cc_core::Document::new(project)), id)
    }

    #[test]
    fn wall_draft_is_isolated_until_saved_as_a_new_type() {
        let (mut state, id) = sample();
        let original_type = state.project().element(id).unwrap().type_id;
        let original_thickness = state.project().element_type(original_type).unwrap().thickness();
        open(&mut state, id);
        let mut editor = state.dialogs.wall_editor.take().unwrap();
        if let TypeData::Wall { layers, .. } = &mut editor.ty.data {
            layers[0].thickness += 0.03;
        }
        assert_eq!(state.project().element_type(original_type).unwrap().thickness(), original_thickness);
        assert!(commit(&mut state, &editor, true), "{}", state.message);
        let assigned = state.project().element(id).unwrap().type_id;
        assert_ne!(assigned, original_type);
        assert!((state.project().element_type(assigned).unwrap().thickness() - original_thickness - 0.03).abs() < 1e-9);
        assert_eq!(state.project().element_type(original_type).unwrap().thickness(), original_thickness);
    }

    #[test]
    fn saving_current_type_updates_the_wall_and_its_type() {
        let (mut state, id) = sample();
        let old_height = state.project().element(id).unwrap().as_wall().unwrap().height;
        open(&mut state, id);
        let mut editor = state.dialogs.wall_editor.take().unwrap();
        editor.wall.height += 0.25;
        if let TypeData::Wall { layers, .. } = &mut editor.ty.data {
            layers[0].thickness += 0.01;
        }
        assert!(commit(&mut state, &editor, false));
        let element = state.project().element(id).unwrap();
        assert!((element.as_wall().unwrap().height - old_height - 0.25).abs() < 1e-9);
        assert_eq!(element.type_id, editor.ty.id);
        assert_eq!(state.project().element_type(editor.ty.id).unwrap().data, editor.ty.data);
    }

    #[test]
    fn type_properties_before_placement_keep_the_original_when_saved_as_new() {
        let (mut state, id) = sample();
        let original = state.project().element(id).unwrap().type_id;
        let original_thickness = state.project().element_type(original).unwrap().thickness();
        state.set_tool(crate::tools::Tool::Wall { type_id: original });
        open_type(&mut state, original);
        let mut editor = state.dialogs.wall_editor.take().unwrap();
        editor.wall.height = 4.0;
        if let TypeData::Wall { layers, .. } = &mut editor.ty.data {
            layers[0].thickness += 0.02;
        }
        assert!(commit(&mut state, &editor, true), "{}", state.message);
        let created = state.tool.type_id().unwrap();
        assert_ne!(created, original);
        assert_eq!(state.project().element(id).unwrap().type_id, original);
        assert_eq!(state.project().element_type(original).unwrap().thickness(), original_thickness);
        assert!((state.project().element_type(created).unwrap().thickness() - original_thickness - 0.02).abs() < 1e-9);
        assert_eq!(state.wall_height, 4.0);
    }
}
