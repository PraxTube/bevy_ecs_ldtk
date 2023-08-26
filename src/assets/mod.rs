//! Assets for loading ldtk files.

mod ldtk_asset_plugin;
pub use ldtk_asset_plugin::LdtkAssetPlugin;

mod ldtk_external_level;
pub use ldtk_external_level::LdtkExternalLevel;

mod ldtk_project_with_metadata;
pub use ldtk_project_with_metadata::LdtkProjectWithMetadata;

pub type LdtkProject = ldtk_project_with_metadata::LdtkProjectWithMetadata<LevelMetadata>;

pub type LdtkParentProject =
    ldtk_project_with_metadata::LdtkProjectWithMetadata<ExternalLevelMetadata>;

mod ldtk_project;

mod level_metadata;
pub use level_metadata::{ExternalLevelMetadata, LevelMetadata};

mod level_selection_accessor;
pub use level_selection_accessor::LevelSelectionAccessor;

mod level_indices;
pub use level_indices::LevelIndices;

mod ldtk_project_getters;
pub use ldtk_project_getters::LdtkProjectGetters;
