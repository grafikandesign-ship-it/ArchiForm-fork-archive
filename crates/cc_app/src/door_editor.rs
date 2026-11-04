//! Properties and type editor for hosted doors.
use crate::{
    actions,
    dialog::{self, Style},
    state::AppState,
    t,
};
use cc_core::{Category, ComponentKind, ElementTypeId, TypeData};
use egui::{Context, RichText, Ui};

pub struct Editor {
    type_id: ElementTypeId,
    name: String,
    width: f64,
    height: f64,
    mark: String,
    description: String,
    manufacturer: String,
    cost: f64,
}

pub fn open_type(state: &mut AppState, id: ElementTypeId) {
    let Some(ty) = state.project().element_type(id).filter(|x| x.category() == Category::Door).cloned() else { return };
    let TypeData::Component { def, .. } = &ty.data else { return };
    if def.kind != ComponentKind::Door {
        return;
    };
    state.last_door_type = Some(id);
    let (width, height) = def.opening_size().unwrap_or((0.8, 2.1));
    state.dialogs.door_editor = Some(Editor {
        type_id: id,
        name: ty.name,
        width,
        height,
        mark: ty.info.type_mark,
        description: ty.info.description,
        manufacturer: ty.info.manufacturer,
        cost: ty.info.cost.unwrap_or(0.0),
    });
}

pub fn show(ctx: &Context, state: &mut AppState) {
    let Some(mut e) = state.dialogs.door_editor.take() else { return };
    let Some(ty) = state.project().element_type(e.type_id).cloned() else { return };
    let mut open = true;
    let mut clicked = None;
    egui::Window::new(t!("door-properties-title"))
        .id(egui::Id::new("door-properties-floating"))
        .default_size([820.0, 760.0])
        .min_size([580.0, 460.0])
        .resizable(true)
        .movable(true)
        .collapsible(false)
        .open(&mut open)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical().max_height((ui.ctx().content_rect().height() - 130.0).max(260.0)).show(
                ui,
                |ui| {
                    body(ui, &mut e, &ty);
                },
            );
            ui.separator();
            ui.horizontal(|ui| {
                if dialog::styled_button(ui, &t!("wall-editor-save-new"), Style::Secondary).clicked() {
                    clicked = Some(1);
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if dialog::styled_button(ui, &t!("action-cancel"), Style::Secondary).clicked() {
                        clicked = Some(2);
                    }
                    if dialog::styled_button(ui, &t!("action-save"), Style::Primary).clicked() {
                        clicked = Some(0);
                    }
                });
            });
        });
    let close = match clicked {
        Some(0) => commit(state, &e, &ty, false),
        Some(1) => commit(state, &e, &ty, true),
        Some(_) => true,
        None => !open,
    };
    if !close && open {
        state.dialogs.door_editor = Some(e);
    }
}

fn body(ui: &mut Ui, e: &mut Editor, ty: &cc_core::ElementType) {
    ui.columns(2, |c| {
        c[0].label(RichText::new(&e.name).strong());
        c[0].horizontal(|u| {
            u.label(t!("type-type"));
            u.text_edit_singleline(&mut e.name);
        });
        c[0].add_space(8.);
        c[0].label(RichText::new(t!("door-geometry")).strong());
        number(&mut c[0], t!("prop-width"), &mut e.width);
        number(&mut c[0], t!("prop-height"), &mut e.height);
        c[0].add_space(8.);
        c[0].label(RichText::new(t!("door-bim")).strong());
        field(&mut c[0], t!("door-mark"), &mut e.mark);
        field(&mut c[0], t!("door-description"), &mut e.description);
        field(&mut c[0], t!("door-manufacturer"), &mut e.manufacturer);
        number(&mut c[0], t!("door-cost"), &mut e.cost);
        c[1].label(RichText::new(t!("comp-preview-3d")).strong());
        c[1].add_space(8.);
        if let TypeData::Component { def, .. } = &ty.data {
            let mut draft = def.clone();
            if let Some(p) = draft.params.get_mut(0) {
                p.value = e.width;
            }
            if let Some(p) = draft.params.get_mut(1) {
                p.value = e.height;
            }
            crate::component_editor::preview_definition(&mut c[1], &draft, usize::MAX);
        }
    });
}
fn number(ui: &mut Ui, label: String, v: &mut f64) {
    ui.horizontal(|u| {
        u.label(label);
        u.add(egui::DragValue::new(v).speed(0.01).range(0.0..=20.0).suffix(" m"));
    });
}
fn field(ui: &mut Ui, label: String, v: &mut String) {
    ui.horizontal(|u| {
        u.label(label);
        u.text_edit_singleline(v);
    });
}
fn commit(s: &mut AppState, e: &Editor, ty: &cc_core::ElementType, new: bool) -> bool {
    if e.name.trim().is_empty() || !e.width.is_finite() || !e.height.is_finite() || e.width <= 0. || e.height <= 0. {
        s.message = t!("door-invalid-properties");
        return false;
    }
    let target = if new {
        let TypeData::Component { def, .. } = &ty.data else { return false };
        let mut def = def.clone();
        if let Some(p) = def.params.get_mut(0) {
            p.value = e.width;
        }
        if let Some(p) = def.params.get_mut(1) {
            p.value = e.height;
        }
        let component = format!("archiform-door-custom-{}", cc_core::Uid::generate());
        let Ok(def) = serde_json::to_value(def) else { return false };
        let cmd = cc_engine::commands::types::ComponentCreate { name: component, def };
        let Some(id) = actions::run_command(s, cmd).and_then(|v| v["id"].as_u64()) else { return false };
        ElementTypeId(id)
    } else {
        e.type_id
    };
    let info = |t: &mut cc_core::ElementType| {
        t.name = e.name.trim().into();
        t.info.type_mark = e.mark.clone();
        t.info.description = e.description.clone();
        t.info.manufacturer = e.manufacturer.clone();
        t.info.cost = (e.cost > 0.).then_some(e.cost);
        if !new && let TypeData::Component { def, .. } = &mut t.data {
            if let Some(p) = def.params.get_mut(0) {
                p.value = e.width;
            }
            if let Some(p) = def.params.get_mut(1) {
                p.value = e.height;
            }
        }
    };
    if new
        || e.name != ty.name
        || e.width != ty.opening_size().unwrap_or_default().0
        || e.height != ty.opening_size().unwrap_or_default().1
    {
        crate::engine_edit::element_type(s, target, None, info);
    }
    if new {
        s.tool = s.tool.with_type(target);
        s.last_door_type = Some(target);
    }
    true
}
