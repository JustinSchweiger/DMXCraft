//! CONTROL: scene grid, stage plan (selection) and programmer.

pub mod audio_trigger_ui;
pub mod fx_color;
pub mod fx_ui;
pub mod programmer;
pub mod scene_grid;
pub mod scene_groups;
pub mod scene_props;
pub mod stage;
pub mod timeline;

pub use programmer::ProgrammerView;
pub use scene_grid::SceneGridView;
pub use scene_props::ScenePropsView;
pub use stage::StageView;
