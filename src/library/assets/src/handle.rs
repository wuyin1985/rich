use super::{
    asset::{AssetPathId, Asset},
};
use std::sync::mpsc::Sender;
use std::marker::PhantomData;
use std::hash::{Hash, Hasher};
use crate::asset::AssetPath;
use bevy_reflect::Uuid;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum HandleId {
    //为不是通过读取文件生成的资源预留
    Id(Uuid, u64),
    AssetPathId(AssetPathId),
}

impl From<AssetPathId> for HandleId {
    fn from(value: AssetPathId) -> Self {
        HandleId::AssetPathId(value)
    }
}

impl<'a> From<AssetPath<'a>> for HandleId {
    fn from(value: AssetPath<'a>) -> Self {
        HandleId::AssetPathId(AssetPathId::from(&value))
    }
}

#[derive(Debug)]
pub struct Handle<T> where T: Asset {
    id: HandleId,
    marker: PhantomData<T>,
}

impl<T: Asset> From<Handle<T>> for HandleId {
    fn from(value: Handle<T>) -> Self {
        value.id
    }
}

impl<T: Asset> Handle<T> {
    pub fn new(id: HandleId) -> Self {
        Self {
            id,
            marker: PhantomData::default(),
        }
    }
}


#[derive(Debug, PartialEq, Clone)]
pub struct HandleUntyped {
    pub id: HandleId,
}

impl HandleUntyped {
    pub fn typed<T: Asset>(mut self) -> Handle<T> {
        if let HandleId::Id(type_uuid, _) = self.id {
            if T::TYPE_UUID != type_uuid {
                panic!("Attempted to convert handle to invalid type.");
            }
        }
        Handle {
            id: self.id,
            marker: PhantomData::default(),
        }
    }
}

impl From<&HandleUntyped> for HandleId {
    fn from(value: &HandleUntyped) -> Self {
        value.id
    }
}

// 
// impl Hash for HandleUntyped {
//     fn hash<H: Hasher>(&self, state: &mut H) {
//         Hash::hash(&self.id, state);
//     }
// }

