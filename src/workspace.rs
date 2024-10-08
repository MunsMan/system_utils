use std::fmt::Display;

use hyprland::data::{Monitor, Workspace, Workspaces};
use hyprland::prelude::*;

use crate::error::WorkspaceError;
use crate::eww::{eww_update, EwwVariable};

pub fn eww_workspaces(default_spaces: usize) -> Result<String, WorkspaceError> {
    let workspaces = Workspaces::get().map_err(WorkspaceError::Hyprland)?;
    Ok(EwwWorkspaces::new(workspaces, default_spaces).to_string())
}

pub fn eww_workspace_update(default_spaces: usize) -> Result<(), WorkspaceError> {
    eww_update(EwwVariable::Workspace(eww_workspaces(default_spaces)?))
        .map_err(WorkspaceError::Command)
}

struct EwwWorkspaces(Vec<EwwWorkspaceButton>);

impl EwwWorkspaces {
    fn new(workspaces: Workspaces, default_spaces: usize) -> Self {
        let mut buttons = workspaces
            .iter()
            .map(|x| x.into())
            .collect::<Vec<EwwWorkspaceButton>>();
        buttons.sort_by(|a, b| a.id.cmp(&b.id));
        let mut results = Vec::new();

        for i in 0..(default_spaces) {
            if let Ok(i) = buttons.binary_search_by_key(&((i + 1) as i32), |x| x.id) {
                results.push(buttons[i].clone());
            } else {
                results.push(EwwWorkspaceButton::empty(i as i32 + 1))
            }
        }
        Self(results)
    }
}

#[derive(Clone)]
struct EwwWorkspaceButton {
    id: i32,
    state: EwwWorkspaceButtonState,
}

#[derive(Clone)]
enum EwwWorkspaceButtonState {
    Emtpy,
    Active,
    Contains,
}

impl EwwWorkspaceButton {
    fn empty(id: i32) -> Self {
        Self {
            id,
            state: EwwWorkspaceButtonState::Emtpy,
        }
    }
}

fn focus_workspace(workspace_id: i32) -> String {
    format!("hyprctl dispatch workspace {}", workspace_id,)
}

impl From<&Workspace> for EwwWorkspaceButton {
    fn from(value: &Workspace) -> Self {
        let mut state = if value.windows == 0 {
            EwwWorkspaceButtonState::Emtpy
        } else {
            EwwWorkspaceButtonState::Contains
        };
        if Monitor::get_active().unwrap().active_workspace.id == value.id {
            state = EwwWorkspaceButtonState::Active;
        };
        EwwWorkspaceButton {
            id: value.id,
            state,
        }
    }
}

impl Display for EwwWorkspaceButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(button :onclick \"{}\" :class \"workspace\"  (box :class \"workspace-{}\" :space-evenly \"false\" \"{}\"))",
            focus_workspace(self.id),
            match self.state {
                EwwWorkspaceButtonState::Emtpy => "empty",
                EwwWorkspaceButtonState::Active => "active",
                EwwWorkspaceButtonState::Contains => "contains",
            },
            match self.state {
                EwwWorkspaceButtonState::Emtpy => '',
                EwwWorkspaceButtonState::Active => '',
                EwwWorkspaceButtonState::Contains => '',
            }
        )
    }
}

impl Display for EwwWorkspaces {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(box :class \"workspaces\" :orientation \"h\" :space-evenly \"false\" {} )",
            self.0
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        )
    }
}
