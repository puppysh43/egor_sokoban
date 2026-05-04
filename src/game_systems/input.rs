use crate::game_systems::MessageOfIntent;
use egor::input::*;
use egor::math::IVec2;

pub fn system(input: &mut &Input) -> MessageOfIntent {
    if input.all_keys_pressed(&[KeyCode::ArrowLeft, KeyCode::ControlLeft])
        || input.all_keys_pressed(&[KeyCode::ArrowLeft, KeyCode::ControlRight])
    {
        return MessageOfIntent::Rewind;
    } else if input.all_keys_pressed(&[KeyCode::ArrowRight, KeyCode::ControlLeft])
        || input.all_keys_pressed(&[KeyCode::ArrowRight, KeyCode::ControlRight])
    {
        return MessageOfIntent::Forward;
    } else if input.keys_pressed(&[KeyCode::ArrowLeft, KeyCode::Numpad4]) {
        return MessageOfIntent::MovePlayer(IVec2::new(-1, 0));
    } else if input.keys_pressed(&[KeyCode::ArrowRight, KeyCode::Numpad6]) {
        return MessageOfIntent::MovePlayer(IVec2::new(1, 0));
    } else if input.keys_pressed(&[KeyCode::ArrowUp, KeyCode::Numpad8]) {
        return MessageOfIntent::MovePlayer(IVec2::new(0, -1));
    } else if input.keys_pressed(&[KeyCode::ArrowDown, KeyCode::Numpad2]) {
        return MessageOfIntent::MovePlayer(IVec2::new(0, 1));
    } else if input.all_keys_pressed(&[KeyCode::KeyQ, KeyCode::ControlLeft])
        || input.keys_pressed(&[KeyCode::KeyQ, KeyCode::ControlRight])
    {
        return MessageOfIntent::Quit;
    } else if input.all_keys_pressed(&[KeyCode::KeyR, KeyCode::ControlLeft])
        || input.all_keys_pressed(&[KeyCode::KeyR, KeyCode::ControlRight])
    {
        return MessageOfIntent::Reset;
    } else {
        return MessageOfIntent::None;
    }
}
