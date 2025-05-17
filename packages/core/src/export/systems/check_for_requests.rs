use crate::export::ExportRequest;
use crate::export::ongoing::OngoingExport;
use crate::states::DungeonRsState;
use bevy::prelude::{Assets, Commands, EventReader, Image, NextState, ResMut};

/// This system checks for incoming [ExportRequest]s and initializes the [OngoingExport].
pub fn check_for_requests(
    mut requests: EventReader<ExportRequest>,
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut dungeonrs_state: ResMut<NextState<DungeonRsState>>,
) {
    let Some(request) = requests.read().next() else {
        return;
    };

    dungeonrs_state.set(DungeonRsState::Loading);
    commands.insert_resource(OngoingExport::new(request, &mut images));
}
