use bevy::{
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy::asset::{AssetLoader, LoadContext, LoadedAsset};
use quick_protobuf::{BytesReader, MessageRead};
use super::proto::PathEditor::*;

#[derive(Debug, TypeUuid)]
#[uuid = "dcb91f31-722c-40e3-862c-eaa05e872436"]
pub struct MapConfigAsset {
    pub config: MapConfig,
}

#[derive(Default)]
pub struct MapConfigAssetLoader;

impl AssetLoader for MapConfigAssetLoader {
    fn load<'a>(&'a self, bytes: &'a [u8], load_context: &'a mut LoadContext) -> BoxedFuture<'a, anyhow::Result<(), anyhow::Error>> {
        Box::pin(async move {
            let mut reader = BytesReader::from_bytes(bytes);
            let config = MapConfig::from_reader(&mut reader, bytes).expect("Cannot read MapConfig");
            let asset = MapConfigAsset { config };
            load_context.set_default_asset(LoadedAsset::new(asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["map"]
    }
}

