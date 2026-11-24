use bevy::prelude::*;

use crate::{
    components::speakers::{Speaker, SpeakerOrder, UpdateSpeakers},
    constants::{
        speakers::{FIRST_ORDER_POS, SECOND_ORDER_POS},
        world::FACE_HEIGHT,
    },
};

pub fn update_speakers(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut update_speakers_reader: MessageReader<UpdateSpeakers>,
    speaker_query: Query<Entity, With<Speaker>>,
) {
    for id in &speaker_query {
        commands.entity(id).despawn();
    }

    let speaker_handle = asset_server.load(
        GltfAssetLabel::Primitive {
            mesh: 0,
            primitive: 0,
        }
        .from_asset("3d/speaker.gltf"),
    );

    for pos in match update_speakers_reader.read().last().unwrap().0 {
        SpeakerOrder::First => FIRST_ORDER_POS.to_vec(),
        SpeakerOrder::Second => SECOND_ORDER_POS.to_vec(),
    } {
        let pos = pos + FACE_HEIGHT;
        commands.spawn((
            Speaker,
            Mesh3d(speaker_handle.clone()),
            MeshMaterial3d(materials.add(Color::srgb_u8(255, 255, 255))),
            Transform::from_xyz(pos.x, pos.y, pos.z).looking_at(FACE_HEIGHT, Dir3::Y),
        ));
    }
}
