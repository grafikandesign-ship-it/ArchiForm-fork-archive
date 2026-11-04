//! Door types and simple, project-owned door presets.
use crate::{actions, i18n::tr, state::AppState, t, theme};
use cc_core::{Category, ComponentKind, TypeData};
use egui::Ui;

#[derive(Clone, Copy, PartialEq, Eq)]
enum DoorGroup {
    Interior,
    Exterior,
    Double,
    Sliding,
    Glazed,
}

impl DoorGroup {
    const ALL: [Self; 5] = [Self::Interior, Self::Exterior, Self::Double, Self::Sliding, Self::Glazed];
    fn label(self) -> &'static str {
        match self {
            Self::Interior => "door-group-interior",
            Self::Exterior => "door-group-exterior",
            Self::Double => "door-group-double",
            Self::Sliding => "door-group-sliding",
            Self::Glazed => "door-group-glazed",
        }
    }
    fn key(self) -> &'static str {
        match self {
            Self::Interior => "interior",
            Self::Exterior => "exterior",
            Self::Double => "double",
            Self::Sliding => "sliding",
            Self::Glazed => "glazed",
        }
    }
}

pub fn show(ui: &mut Ui, state: &mut AppState) {
    restore_legacy_default_doors(state);
    if !state.project().types.values().any(|ty| {
        matches!(&ty.data, TypeData::Component { component, .. } if component.starts_with("archiform-door-preset-"))
    }) {
        seed_library(state);
    }
    theme::panel_header(ui, &t!("door-library-title"), None);
    egui::ScrollArea::vertical().id_salt("door-library").auto_shrink([false, false]).show(ui, |ui| {
        ui.add_space(5.0);
        for group in DoorGroup::ALL {
            let types: Vec<_> = state
                .project()
                .types_of(Category::Door)
                .filter_map(|ty| {
                    let TypeData::Component { component, def } = &ty.data else { return None };
                    let component_key = format!("archiform-door-preset-{}-", group.key());
                    (def.kind == ComponentKind::Door && component.starts_with(&component_key))
                        .then(|| (ty.id, ty.name.clone(), ty.opening_size().unwrap_or_default()))
                })
                .collect();
            egui::CollapsingHeader::new(tr(group.label()))
                .default_open(group == DoorGroup::Interior || group == DoorGroup::Exterior)
                .show(ui, |ui| {
                    for (id, name, (w, h)) in types {
                        let selected = state.tool.type_id() == Some(id);
                        let response =
                            ui.selectable_label(selected, format!("{name} · {:.0}×{:.0}", w * 100.0, h * 100.0));
                        if response.clicked() {
                            state.tool = state.tool.with_type(id);
                            state.last_door_type = Some(id);
                        }
                        let mut edit_type = false;
                        response.context_menu(|ui| {
                            if ui.button(t!("props-edit-type")).clicked() {
                                edit_type = true;
                                ui.close();
                            }
                        });
                        if edit_type {
                            crate::door_editor::open_type(state, id);
                        }
                    }
                });
        }
        let others: Vec<_> = state
            .project()
            .types_of(Category::Door)
            .filter_map(|ty| {
                let TypeData::Component { component, def } = &ty.data else { return None };
                (def.kind == ComponentKind::Door
                    && !component.starts_with("archiform-door-preset-")
                    && !is_legacy_component(component))
                .then(|| (ty.id, ty.name.clone(), ty.opening_size().unwrap_or_default()))
            })
            .collect();
        egui::CollapsingHeader::new(tr("door-group-other")).default_open(false).show(ui, |ui| {
            for (id, name, (w, h)) in others {
                let response = ui.selectable_label(
                    state.tool.type_id() == Some(id),
                    format!("{name} · {:.0}×{:.0}", w * 100.0, h * 100.0),
                );
                if response.clicked() {
                    state.tool = state.tool.with_type(id);
                    state.last_door_type = Some(id);
                }
                let mut edit_type = false;
                response.context_menu(|ui| {
                    if ui.button(t!("props-edit-type")).clicked() {
                        edit_type = true;
                        ui.close();
                    }
                });
                if edit_type {
                    crate::door_editor::open_type(state, id);
                }
            }
        });
        if let Some(id) = state.tool.type_id()
            && ui.button(t!("props-edit-type")).clicked()
        {
            crate::door_editor::open_type(state, id);
        }
        ui.separator();
        if ui.button(t!("door-create-presets")).clicked() {
            seed_library(state);
        }
        ui.label(theme::muted(t!("door-library-hint")).size(11.0));
    });
}

fn seed_library(state: &mut AppState) {
    if state.project().types_of(Category::Door).next().is_none() {
        state.message = t!("door-library-no-types");
        return;
    }
    let presets = [
        (DoorGroup::Interior, 0.70, 2.10, "Interior 70 × 210"),
        (DoorGroup::Interior, 0.80, 2.10, "Interior 80 × 210"),
        (DoorGroup::Interior, 0.90, 2.10, "Interior 90 × 210"),
        (DoorGroup::Exterior, 0.90, 2.10, "Exterior 90 × 210"),
        (DoorGroup::Exterior, 1.00, 2.10, "Exterior 100 × 210"),
        (DoorGroup::Double, 1.20, 2.10, "Double 120 × 210"),
        (DoorGroup::Double, 1.40, 2.10, "Double 140 × 210"),
        (DoorGroup::Sliding, 0.80, 2.10, "Sliding 80 × 210"),
        (DoorGroup::Sliding, 0.90, 2.10, "Sliding 90 × 210"),
        (DoorGroup::Glazed, 0.80, 2.10, "Glazed 80 × 210"),
        (DoorGroup::Glazed, 0.90, 2.10, "Glazed 90 × 210"),
    ];
    let glass_material = state
        .project()
        .materials
        .iter()
        .find(|m| m.identity.class == cc_core::material::MaterialClass::Glass)
        .map(|m| m.name.clone())
        .unwrap_or_default();
    for (group, w, h, name) in presets {
        let component = format!("archiform-door-preset-{}-{:03}-210", group.key(), (w * 100.0_f64).round() as u32);
        if state
            .project()
            .types
            .values()
            .any(|ty| matches!(&ty.data, TypeData::Component { component: c, .. } if c == &component))
        {
            continue;
        }
        let def = door_definition(group, w, h, &glass_material);
        let Ok(def) = serde_json::to_value(def) else { continue };
        let command = cc_engine::commands::types::ComponentCreate { name: component, def };
        let Some(id) = actions::run_command(state, command).and_then(|v| v["id"].as_u64()) else { continue };
        let tid = cc_core::ElementTypeId(id);
        let mut display_name = name.to_string();
        let mut suffix = 1;
        while state.project().types_of(Category::Door).any(|ty| ty.name == display_name) {
            suffix += 1;
            display_name =
                if suffix == 2 { format!("{name} · ArchiForm") } else { format!("{name} · ArchiForm {suffix}") };
        }
        crate::engine_edit::element_type(state, tid, None, |ty| ty.name = display_name);
    }
}

const LEGACY_PRESET_NAMES: [&str; 11] = [
    "Interior 70 × 210",
    "Interior 80 × 210",
    "Interior 90 × 210",
    "Exterior 90 × 210",
    "Exterior 100 × 210",
    "Double 120 × 210",
    "Double 140 × 210",
    "Sliding 80 × 210",
    "Sliding 90 × 210",
    "Glazed 80 × 210",
    "Glazed 90 × 210",
];

fn is_legacy_component(component: &str) -> bool {
    LEGACY_PRESET_NAMES.contains(&component)
}

fn restore_legacy_default_doors(state: &mut AppState) {
    let default_doors = [
        ("Door 80x210", 0.8),
        ("Door 90x210", 0.9),
        ("Tür 80x210", 0.8),
        ("Tür 90x210", 0.9),
        ("Puerta 80x210", 0.8),
        ("Puerta 90x210", 0.9),
        ("Porte 80x210", 0.8),
        ("Porte 90x210", 0.9),
        ("Porta 80x210", 0.8),
        ("Porta 90x210", 0.9),
        ("ປະຕູ 80x210", 0.8),
        ("ປະຕູ 90x210", 0.9),
        ("Drzwi 80x210", 0.8),
        ("Drzwi 90x210", 0.9),
        ("Двері 80 × 210 см", 0.8),
        ("Двері 90 × 210 см", 0.9),
        ("门 80x210", 0.8),
        ("门 90x210", 0.9),
        ("門 80x210", 0.8),
        ("門 90x210", 0.9),
    ];
    let legacy: Vec<_> = state
        .project()
        .types_of(Category::Door)
        .filter_map(|ty| {
            let TypeData::Component { component, def } = &ty.data else { return None };
            let default_width = default_doors.iter().find(|(name, _)| ty.name == *name).map(|(_, w)| *w)?;
            (is_legacy_component(component)
                && def.kind == ComponentKind::Door
                && def.items.iter().any(|item| item.name.to_lowercase().contains("leaf b")))
            .then(|| (ty.clone(), default_width))
        })
        .collect();
    if legacy.is_empty() || state.project().types_of(Category::Door).count() <= 1 {
        return;
    }
    for (old, default_width) in legacy {
        let TypeData::Component { def: old_def, .. } = &old.data else { continue };
        let (width, height) = old_def.opening_size().unwrap_or((default_width, 2.1));
        let component = format!("archiform-door-restored-{}", cc_core::Uid::generate());
        let def = door_definition(DoorGroup::Interior, width, height, "");
        let Ok(def) = serde_json::to_value(def) else { continue };
        let command = cc_engine::commands::types::ComponentCreate { name: component, def };
        let Some(new_id) = actions::run_command(state, command).and_then(|v| v["id"].as_u64()) else { continue };
        let new_id = cc_core::ElementTypeId(new_id);
        let elements: Vec<_> =
            state.project().elements.values().filter(|e| e.type_id == old.id).map(|e| e.id).collect();
        for id in elements {
            crate::engine_edit::element(state, id, None, |e| e.type_id = new_id);
        }
        let deleted = actions::run_command(
            state,
            cc_engine::commands::types::TypeDelete { type_ref: cc_engine::Ref::Id(old.id.0) },
        )
        .is_some();
        if deleted {
            crate::engine_edit::element_type(state, new_id, None, |ty| {
                ty.name = old.name.clone();
                ty.info = old.info.clone();
            });
            if state.tool.type_id() == Some(old.id) {
                state.tool = state.tool.with_type(new_id);
            }
        }
    }
}

fn door_definition(group: DoorGroup, w: f64, h: f64, glass_material: &str) -> cc_core::ComponentDef {
    let names = cc_core::openings::OpeningNames::default();
    let mut def = cc_core::openings::door_def(&names, w, h, "");
    let width = &names.width;
    let height = &names.height;
    let thickness = cc_core::component::WALL_THICKNESS;
    let frame = 0.055;
    let box_item = |name: &str, material: &str, x: String, y: String, z: String, dx: String, dy: String, dz: String| {
        cc_core::ComponentItem {
            name: name.into(),
            material: material.into(),
            in_plan: false,
            void: false,
            plan_only: false,
            shape: cc_core::ComponentShape::Box { x, y, z, width: dx, depth: dy, height: dz },
        }
    };
    let expr = |s: &str| {
        s.replace("{W}", width).replace("{H}", height).replace("{T}", thickness).replace("{F}", &frame.to_string())
    };
    let mut items = vec![
        box_item("Frame", "", expr("-{W}/2"), expr("-{T}/2"), "0".into(), expr("{F}"), expr("{T}"), height.clone()),
        box_item("Frame", "", expr("{W}/2-{F}"), expr("-{T}/2"), "0".into(), expr("{F}"), expr("{T}"), height.clone()),
        box_item(
            "Head",
            "",
            expr("-{W}/2+{F}"),
            expr("-{T}/2"),
            expr("{H}-{F}"),
            expr("{W}-2*{F}"),
            expr("{T}"),
            expr("{F}"),
        ),
    ];
    let leaves = if matches!(group, DoorGroup::Double | DoorGroup::Glazed | DoorGroup::Sliding) { 2 } else { 1 };
    for i in 0..leaves {
        let (x, leaf_w) = if leaves == 2 {
            if i == 0 {
                ("-{W}/2+{F}".to_string(), "{W}/2-{F}".to_string())
            } else {
                ("0".to_string(), "{W}/2-{F}".to_string())
            }
        } else {
            ("-{W}/2+{F}".into(), "{W}-2*{F}".into())
        };
        let label = if group == DoorGroup::Sliding {
            format!("Sliding leaf {}", if i == 0 { "A" } else { "B" })
        } else if leaves == 2 {
            format!("Leaf {}", if i == 0 { "A" } else { "B" })
        } else {
            "Leaf".into()
        };
        if group == DoorGroup::Glazed {
            let inner_x = format!("{x}+{{F}}");
            let inner_w = format!("{leaf_w}-2*{{F}}");
            for (part, part_x, part_z, part_w, part_h) in [
                ("left stile", x.clone(), "0".to_string(), "{F}".to_string(), "{H}".to_string()),
                ("right stile", format!("{x}+{leaf_w}-{{F}}"), "0".to_string(), "{F}".to_string(), "{H}".to_string()),
                ("bottom rail", inner_x.clone(), "0".to_string(), inner_w.clone(), "{F}".to_string()),
                ("top rail", inner_x.clone(), "{H}-{F}".to_string(), inner_w.clone(), "{F}".to_string()),
            ] {
                items.push(box_item(
                    &format!("{label} {part}"),
                    "",
                    expr(&part_x),
                    "-0.03".into(),
                    expr(&part_z),
                    expr(&part_w),
                    "0.06".into(),
                    expr(&part_h),
                ));
            }
            items.push(box_item(
                &format!("{label} glazing"),
                glass_material,
                expr(&inner_x),
                "-0.008".into(),
                expr("{F}"),
                expr(&inner_w),
                "0.016".into(),
                expr("{H}-2*{F}"),
            ));
        } else {
            items.push(box_item(
                &label,
                "",
                expr(&x),
                "-0.02".into(),
                "0".into(),
                expr(&leaf_w),
                "0.04".into(),
                expr("{H}"),
            ));
        }
    }
    def.items = items;
    def
}
