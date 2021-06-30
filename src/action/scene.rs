use crate::app::Application;

use super::{Action, ActionType};

pub enum SceneAction {
    Forward(bool),
    Backward(bool),
    Left(bool),
    Right(bool),
    Up(bool),
    Down(bool),
    Pan(f32, f32),
}

impl ActionType for SceneAction {
    fn perform(self, app: &mut Application) {
        app.frame.input.update(self);
    }
}

impl From<SceneAction> for Action {
    fn from(action: SceneAction) -> Self {
        Action::Scene(action)
    }
}
