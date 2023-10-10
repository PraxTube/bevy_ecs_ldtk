use crate::{
    assets::{LdtkJsonWithMetadata, LevelMetadata, LevelMetadataAccessor},
    ldtk::{LdtkJson, Level},
    prelude::RawLevelAccessor,
};
use bevy::reflect::Reflect;
use derive_more::{From, TryInto};

#[cfg(feature = "internal_levels")]
use crate::assets::InternalLevels;

#[cfg(feature = "external_levels")]
use crate::assets::ExternalLevels;

/// LDtk json data and level metadata for both internal- and external-level projects.
///
/// We need to abstract over these cases to allow them in the same asset type: [`LdtkProject`].
/// All methods that are available in both cases are available here.
/// However, methods exclusive to each case require accessing the internal type.
/// These include methods for obtaining [`LoadedLevel`]s.
/// See the [`LoadedLevel`]-accessing methods in the following impls:
/// - [standalone projects](LdtkJsonWithMetadata#impl-LdtkJsonWithMetadata<InternalLevels>)
/// - [parent projects](LdtkJsonWithMetadata#impl-LdtkJsonWithMetadata<ExternalLevels>)
///
/// Note that this type's variants are under different feature flags.
/// At least one of these feature flags needs to be enabled for the plugin to compile.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
/// [`LoadedLevel`]: crate::ldtk::loaded_level::LoadedLevel
#[derive(Clone, Debug, PartialEq, From, TryInto, Reflect)]
#[try_into(owned, ref)]
pub enum LdtkProjectData {
    /// LDtk data for a standalone project (uses internal levels).
    ///
    /// This is only available under the `internal_levels` feature.
    #[cfg(feature = "internal_levels")]
    Standalone(LdtkJsonWithMetadata<InternalLevels>),
    /// LDtk data for a parent project (uses external levels).
    ///
    /// This is only available under the `external_levels` feature.
    #[cfg(feature = "external_levels")]
    Parent(LdtkJsonWithMetadata<ExternalLevels>),
}

impl LdtkProjectData {
    /// Raw ldtk json data.
    pub fn json_data(&self) -> &LdtkJson {
        match self {
            #[cfg(feature = "internal_levels")]
            LdtkProjectData::Standalone(project) => project.json_data(),
            #[cfg(feature = "external_levels")]
            LdtkProjectData::Parent(project) => project.json_data(),
        }
    }

    /// Unwrap as a [`LdtkJsonWithMetadata<InternalLevels>`].
    /// For use on internal-levels ldtk projects only.
    ///
    /// # Panics
    /// Panics if this is not [`LdtkProjectData::Standalone`].
    /// This shouldn't occur if the project uses internal levels.
    ///
    /// [`LdtkJsonWithMetadata<InternalLevels>`]: LdtkJsonWithMetadata
    #[cfg(feature = "internal_levels")]
    pub fn as_standalone(&self) -> &LdtkJsonWithMetadata<InternalLevels> {
        self.try_into().unwrap()
    }

    /// Unwrap as a [`LdtkJsonWithMetadata<ExternalLevels>`].
    /// For use on external-levels ldtk projects only.
    ///
    /// # Panics
    /// Panics if this is not [`LdtkProjectData::Parent`].
    /// This shouldn't occur if the project uses external levels.
    ///
    /// [`LdtkJsonWithMetadata<ExternalLevels>`]: LdtkJsonWithMetadata
    #[cfg(feature = "external_levels")]
    pub fn as_parent(&self) -> &LdtkJsonWithMetadata<ExternalLevels> {
        self.try_into().unwrap()
    }
}

impl RawLevelAccessor for LdtkProjectData {
    fn worlds(&self) -> &[crate::ldtk::World] {
        self.json_data().worlds()
    }

    fn root_levels(&self) -> &[Level] {
        self.json_data().root_levels()
    }
}

impl LevelMetadataAccessor for LdtkProjectData {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
        match self {
            #[cfg(feature = "internal_levels")]
            LdtkProjectData::Standalone(project) => project.get_level_metadata_by_iid(iid),
            #[cfg(feature = "external_levels")]
            LdtkProjectData::Parent(project) => project.get_level_metadata_by_iid(iid),
        }
    }
}

#[cfg(test)]
#[cfg(feature = "internal_levels")]
mod internal_level_tests {
    use crate::ldtk::fake::{MixedLevelsLdtkJsonFaker, UnloadedLevelsFaker};

    use super::*;
    use fake::{Dummy, Fake, Faker};

    impl Dummy<InternalLevels> for LdtkProjectData {
        fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &InternalLevels, rng: &mut R) -> Self {
            LdtkProjectData::Standalone(Faker.fake_with_rng(rng))
        }
    }

    #[test]
    fn json_data_accessor_is_transparent() {
        let project: LdtkProjectData = InternalLevels.fake();

        assert_eq!(project.json_data(), project.as_standalone().json_data());
    }

    #[test]
    fn raw_level_accessor_implementation_is_transparent() {
        let data: LdtkJson = MixedLevelsLdtkJsonFaker(UnloadedLevelsFaker(4..8), 4..8).fake();

        let project = LdtkProjectData::Standalone(LdtkJsonWithMetadata {
            json_data: data.clone(),
            level_map: HashMap::default(),
        });

        assert_eq!(project.root_levels(), data.root_levels());
        assert_eq!(project.worlds(), data.worlds());
    }
}

#[cfg(test)]
#[cfg(feature = "external_levels")]
mod external_level_tests {
    use super::*;
}
