use crate::editor_systems::*;
use crate::editorstate::EditorState;
use egor::input::{Input, KeyCode};

pub fn system(state: &mut EditorState, input: &mut &Input) -> EditorMOI {
    match state.control_state {
        EditorControlState::Root => {
            if input.keys_pressed(&[KeyCode::Enter, KeyCode::NumpadEnter]) {
                state.control_state = EditorControlState::Reticule(IVec2::new(0, 0));
                return EditorMOI::None;
            } else if input.all_keys_pressed(&[KeyCode::ControlLeft, KeyCode::KeyS])
                || input.all_keys_pressed(&[KeyCode::ControlRight, KeyCode::KeyS])
            {
                state.control_state = EditorControlState::Saving;
                return EditorMOI::None;
            } else {
                return EditorMOI::None;
            }
        }
        EditorControlState::Reticule(_pos) => {
            //if the brush shape is a reticule they can move the reticule and place tiles depending on what they've selected
            if input.keys_pressed(&[KeyCode::ArrowLeft, KeyCode::Numpad4]) {
                return EditorMOI::MoveReticule(IVec2::new(-1, 0));
            } else if input.keys_pressed(&[KeyCode::ArrowUp, KeyCode::Numpad8]) {
                return EditorMOI::MoveReticule(IVec2::new(0, -1));
            } else if input.keys_pressed(&[KeyCode::ArrowRight, KeyCode::Numpad6]) {
                return EditorMOI::MoveReticule(IVec2::new(1, 0));
            } else if input.keys_pressed(&[KeyCode::ArrowDown, KeyCode::Numpad2]) {
                return EditorMOI::MoveReticule(IVec2::new(0, 1));
            } else if input.key_pressed(KeyCode::Digit1) {
                //select the wall brushtype
                state.brush_type = BrushType::Wall;
                return EditorMOI::None;
            } else if input.key_pressed(KeyCode::Digit2) {
                //select the floor brushtype
                state.brush_type = BrushType::Floor;
                return EditorMOI::None;
            } else if input.key_pressed(KeyCode::Digit3) {
                //select the crate brushtype
                state.brush_type = BrushType::Crate;
                return EditorMOI::None;
            } else if input.key_pressed(KeyCode::Digit4) {
                state.brush_type = BrushType::LoadingZone;
                return EditorMOI::None;
            } else if input.key_pressed(KeyCode::Digit5) {
                state.brush_type = BrushType::Player;
                return EditorMOI::None;
            } else if input.keys_pressed(&[KeyCode::Enter, KeyCode::NumpadEnter]) {
                return EditorMOI::PaintTile;
            } else if input.key_pressed(KeyCode::Escape) {
                state.control_state = EditorControlState::Root;
                return EditorMOI::None;
                //go from the reticule brushshape and back to None
            } else {
                //otherwise just do nothing
                return EditorMOI::None;
            }
        }
        EditorControlState::Saving => {
            if input.keys_pressed(&[KeyCode::Enter, KeyCode::NumpadEnter]) {
                return EditorMOI::Save;
            } else {
                return EditorMOI::None;
            }
        }
    }
}
