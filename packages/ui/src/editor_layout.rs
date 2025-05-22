use crate::UiState;
use crate::widgets::toolbar::toolbar;
use bevy::diagnostic::DiagnosticsStore;
use bevy::prelude::{EventWriter, Res, With, World, warn};
use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContext, EguiContexts};
use bevy_inspector_egui::bevy_inspector;
use core::prelude::*;
use egui::{Direction, Layout, SidePanel, TopBottomPanel};

/// This system builds the editor layout, positioning all other widgets and panels on the screen.
#[allow(clippy::needless_pass_by_value)]
pub fn editor_layout(
    mut contexts: EguiContexts,
    diagnostics: Res<DiagnosticsStore>,
    ui_state: Res<UiState>,
    export_writer: EventWriter<ExportRequest>,
    save_writer: EventWriter<SaveProjectRequest>,
    load_writer: EventWriter<LoadProjectRequest>,
) {
    let Some(context) = contexts.try_ctx_mut() else {
        warn!("Failed to acquire egui context");
        return;
    };

    toolbar(
        context,
        diagnostics,
        ui_state.logo,
        export_writer,
        save_writer,
        load_writer,
    );

    TopBottomPanel::bottom("bottom_panel")
        .frame(egui::Frame::NONE)
        .show_separator_line(false)
        .show(context, |ui| {
            ui.with_layout(
                Layout::centered_and_justified(Direction::LeftToRight),
                |ui| {
                    ui.label("Hello World");
                },
            );
        });
}

pub fn inspector_layout(world: &mut World) {
    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    SidePanel::right("right_panel")
        .resizable(true)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_world(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
}
