use bevy::prelude::Component;
use egui::Ui;
use std::path::PathBuf;

/// This tracks all data and state related to the creation of a new [`assets::AssetPack`].
///
/// The form fields, pending operations and result status (success/failure) are tracked in this resource.
#[derive(Component)]
pub struct AssetPackBuilder {
    pub name: String,
    pub path: PathBuf,
}

pub fn show_asset_pack_builder(builder: &mut AssetPackBuilder, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Name: ");
        ui.text_edit_singleline(&mut builder.name);
    });
    ui.horizontal(|ui| {
        ui.label("Path: ");
        ui.text_edit_singleline(&mut String::new());
        if ui.button("...").clicked() {
            // let mut file_dialog = FileDialog::select_folder(None);
            // file_dialog.open();
            //
            // state.open_asset_folder = Some(file_dialog);
        }
    });
}
