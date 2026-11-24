use bevy::ecs::{component::Component, message::Message};

#[derive(Component)]
pub struct Speaker;

#[derive(Message, Debug)]
pub struct UpdateSpeakers(pub SpeakerOrder);

#[derive(Debug)]
pub enum SpeakerOrder {
    First,
    Second,
}
