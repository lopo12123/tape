use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::canonicalize::declaration::{ActionType, CanonicalAction, CanonicalButton, CanonicalKey};

pub mod declaration;
pub mod convert_enigo;
pub mod convert_dq;

/// An **action** is a single event that can be performed by an [actor](../act/struct.Actor.html)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Action {
    /// The timestamp of the happening of the action
    pub ctime: i64,
    /// The time since the beginning of the script
    pub timeline: i64,
    /// The action to perform
    pub action: CanonicalAction,
}

impl Action {
    pub fn from_keyboard(ev: ActionType, target: CanonicalKey) -> Action {
        Action {
            ctime: Utc::now().timestamp_millis(),
            timeline: 0,
            action: CanonicalAction::Keyboard(ev, target),
        }
    }

    pub fn from_mouse(ev: ActionType, target: CanonicalButton, pos: (i32, i32)) -> Action {
        Action {
            ctime: Utc::now().timestamp_millis(),
            timeline: 0,
            action: CanonicalAction::Mouse(ev, target, pos.0, pos.1),
        }
    }
}

/// A **script** is a sequence of [action](struct.Action.html)s recorded by a [recorder](../rec/struct.Recorder.html) for an [actor](../act/struct.Actor.html) to perform
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Script {
    /// The timestamp of creation of the script
    pub ctime: i64,
    /// The duration of the script in milliseconds
    pub duration: i64,
    /// The actions to perform
    pub actions: Vec<Action>,
}

impl Script {
    /// Check if the script is valid
    /// Rule:
    /// 1. The actions are sorted by their timeline
    /// 2. The duration of the script should not be less than the timeline of the last action
    fn self_check(&self) -> Result<(), String> {
        // check if the actions are sorted by their timeline
        let mut prev = 0;
        for action in &self.actions {
            if action.timeline < prev {
                return Err(format!("The actions are not sorted by their timeline!"));
            }
            prev = action.timeline;
        }

        // check if the duration of the script is not less than the timeline of the last action
        if self.duration < prev {
            return Err(format!("The duration of the script is less than the timeline of the last action!"));
        }

        Ok(())
    }

    /// Create a empty script
    pub fn empty() -> Script {
        let t = Utc::now();
        Script {
            ctime: t.timestamp_millis(),
            duration: 0,
            actions: Vec::new(),
        }
    }

    /// Load a script from a TOML string.
    /// If the parsing is successful and the self-test passes, the script will be returned,
    /// otherwise an error message will be returned.
    pub fn load(raw: String) -> Result<Script, String> {
        match toml::from_str::<Script>(&raw) {
            Ok(script) => {
                match script.self_check() {
                    Ok(_) => Ok(script),
                    Err(check_err) => Err(check_err),
                }
            }
            Err(parse_err) => Err(format!("{}", parse_err)),
        }
    }

    /// Publish the script as text
    pub fn publish(&self) -> Result<String, String> {
        match self.self_check() {
            Ok(_) => toml::to_string(self).map_err(|e| format!("{}", e)),
            Err(check_err) => Err(check_err),
        }
    }
}


#[cfg(test)]
mod unit_test {
    use super::*;
    use std::thread;
    use std::time::Duration;
    use device_query::Keycode;

    #[test]
    fn script_serde() {
        let mut mv = Script::empty();
        thread::sleep(Duration::from_secs(1));
        mv.add_keyboard_action(ActionType::Press, Keycode::A);
        thread::sleep(Duration::from_secs(2));
        mv.add_keyboard_action(ActionType::Release, Keycode::K);
        thread::sleep(Duration::from_secs(1));
        mv.add_mouse_action(ActionType::Press, 1, (50, 50));
        thread::sleep(Duration::from_secs(1));
        mv.add_mouse_action(ActionType::Release, 0, (50, 50));

        match toml::to_string(&mv) {
            Ok(v) => {
                println!("ok! {}", v);
                match toml::from_str::<Script>(&v) {
                    Ok(v) => println!("ok! {:?}", v),
                    Err(e) => println!("err! {}", e),
                }
            }
            Err(e) => println!("err! {}", e),
        }
    }
}