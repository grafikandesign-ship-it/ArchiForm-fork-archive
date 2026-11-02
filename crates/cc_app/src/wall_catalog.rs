//! Wall types and schematic presets shown beside the plan while drawing.

use crate::{actions, i18n::tr, state::AppState, t, theme};
use cc_core::{Category, ElementTypeId, TypeData, WallFunction};
use cc_engine::commands::wall_presets::{self, WallPresetCategory};
use cc_views::format;
use egui::Ui;

pub fn show(ui: &mut Ui, state: &mut AppState) {
    theme::panel_header(ui, &t!("wall-presets-menu"), None);
    egui::ScrollArea::vertical().id_salt("wall-catalog").auto_shrink([false, false]).show(ui, |ui| {
        ui.add_space(5.0);
        ui.label(t!("type-type"));
        for function in WallFunction::ALL {
            let types: Vec<_> = state
                .project()
                .types_of(Category::Wall)
                .filter(|ty| matches!(&ty.data, TypeData::Wall { function: f, .. } if *f == function))
                .map(|ty| (ty.id, ty.name.clone(), ty.thickness()))
                .collect();
            if types.is_empty() {
                continue;
            }
            egui::CollapsingHeader::new(tr(function.key())).default_open(true).show(ui, |ui| {
                for (id, name, thickness) in &types {
                    let selected = state.tool.type_id() == Some(*id);
                    let label = format!("{name} · {}", format::length(*thickness));
                    if ui.selectable_label(selected, label).clicked() {
                        state.tool = state.tool.with_type(*id);
                    }
                }
            });
        }
        if let Some(id) = state.tool.type_id()
            && ui.button(t!("props-edit-type")).clicked()
        {
            crate::wall_editor::open_type(state, id);
        }
        ui.separator();
        ui.label(t!("wall-presets-menu"));
        for category in WallPresetCategory::ALL {
            egui::CollapsingHeader::new(tr(category.key())).show(ui, |ui| {
                for preset in wall_presets::catalog().iter().filter(|p| p.category == category) {
                    let installed = state
                        .project()
                        .types_of(Category::Wall)
                        .find(|ty| ty.info.type_mark == preset.id)
                        .map(|ty| ty.id);
                    let selected = installed == state.tool.type_id();
                    let label = format!("{} · {}", preset.name(), format::length(preset.thickness()));
                    if ui.selectable_label(selected, label).clicked() {
                        select_preset(state, preset.id);
                    }
                }
            });
        }
        ui.label(theme::muted(t!("wall-presets-hint")).size(11.0));
        if let Some(layers) =
            state.tool.type_id().and_then(|id| state.project().element_type(id)).map(|ty| ty.layers().to_vec())
        {
            ui.separator();
            ui.label(t!("comp-preview"));
            crate::wall_editor::paint_preview(ui, state, &layers, 115.0);
        }
    });
}

fn select_preset(state: &mut AppState, preset: &str) {
    let command = wall_presets::WallPresetAdd { preset: preset.into() };
    if let Some(id) = actions::run_command(state, command).and_then(|out| out["id"].as_u64()) {
        state.tool = state.tool.with_type(ElementTypeId(id));
    }
}
