//! SCENE GROUPS: a named set of scenes, each with a time offset. A group is
//! put into SUPER SCENES as one block (linked: changes to the group apply
//! everywhere, or as a copy). Groups are not played on their own.

use serde::{Deserialize, Serialize};

use crate::show::{SceneId, Show};

/// One scene in a group.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct GroupMember {
    pub scene: SceneId,
    /// Start within the group.
    #[serde(default)]
    pub offset_ms: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SceneGroup {
    pub id: u32,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<[u8; 3]>,
    /// In priority order: the first scene wins where they overlap.
    #[serde(default)]
    pub members: Vec<GroupMember>,
}

/// Length of a group: end of its last scene (one cycle each).
pub fn members_length(show: &Show, members: &[GroupMember]) -> u64 {
    members
        .iter()
        .filter_map(|m| {
            show.scene(m.scene)
                .map(|s| m.offset_ms + s.duration_ms().max(1))
        })
        .max()
        .unwrap_or(0)
}

impl Show {
    pub fn scene_group(&self, id: u32) -> Option<&SceneGroup> {
        self.scene_groups.iter().find(|g| g.id == id)
    }

    pub fn scene_group_mut(&mut self, id: u32) -> Option<&mut SceneGroup> {
        self.scene_groups.iter_mut().find(|g| g.id == id)
    }

    /// New, empty group; returns its ID.
    pub fn add_scene_group(&mut self, name: impl Into<String>) -> u32 {
        let id = self.scene_groups.iter().map(|g| g.id).max().unwrap_or(0) + 1;
        self.scene_groups.push(SceneGroup {
            id,
            name: name.into(),
            color: None,
            members: Vec::new(),
        });
        id
    }

    /// Remove a group; its linked blocks in SUPER SCENES go too.
    pub fn remove_scene_group(&mut self, id: u32) {
        self.scene_groups.retain(|g| g.id != id);
        for tl in self.scenes.iter_mut().filter_map(|s| s.timeline_mut()) {
            for t in &mut tl.tracks {
                t.clips.retain(|c| {
                    !matches!(c.content, crate::timeline::ClipContent::Group { group, members: None } if group == id)
                });
            }
        }
    }

    /// Members of a group block: its own copy or those of the linked group.
    pub fn group_members<'a>(
        &'a self,
        content: &'a crate::timeline::ClipContent,
    ) -> Option<&'a [GroupMember]> {
        match content {
            crate::timeline::ClipContent::Group {
                members: Some(m), ..
            } => Some(m),
            crate::timeline::ClipContent::Group { group, .. } => {
                self.scene_group(*group).map(|g| g.members.as_slice())
            }
            _ => None,
        }
    }

    /// Length of a group (end of its last scene).
    pub fn scene_group_length(&self, id: u32) -> u64 {
        self.scene_group(id)
            .map_or(0, |g| members_length(self, &g.members))
    }
}
