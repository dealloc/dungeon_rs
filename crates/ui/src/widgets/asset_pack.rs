use bevy::prelude::{Component, Mut};
use egui::Ui;

/// This tracks all data and state related to the creation of a new [`assets::AssetPack`].
///
/// The form fields, pending operations and result status (success/failure) are tracked in this resource.
#[derive(Component, Default)]
pub struct AssetPackBuilder {
    pub name: String,
    pub path: String,
}

/// Displays an instance of an [`AssetPackBuilder`] in the given `Ui`.
///
/// This method should be called for each builder, each frame.
pub fn show_asset_pack_builder(mut builder: Mut<AssetPackBuilder>, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Name: ");
        ui.text_edit_singleline(&mut builder.name);
    });
    ui.horizontal(|ui| {
        ui.label("Path: ");
        ui.text_edit_singleline(&mut builder.path);
        if ui.button("...").clicked() {
            // let mut file_dialog = FileDialog::select_folder(None);
            // file_dialog.open();
            //
            // state.open_asset_folder = Some(file_dialog);
        }
    });
}
