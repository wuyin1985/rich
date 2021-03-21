use super::{
    asset::{AssetPathId, Asset},
};
use std::sync::mpsc::Sender;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub enum HandleId {
    //为不是通过读取文件生成的文件预留
    Id(u64),
    AssetPathId(AssetPathId),
}

impl From<AssetPathId> for HandleId {
    fn from(value: AssetPathId) -> Self {
        HandleId::AssetPathId(value)
    }
}

enum HandleType {
    Weak,
    Strong(Sender<RefChange>),
}

pub enum RefChange {
    Increment(HandleId),
    Decrement(HandleId),
}

pub struct Handle<T> where T: Asset {
    id: HandleId,
    handle_type: HandleType,
    marker: PhantomData<T>,
}

impl<T: Asset> Drop for Handle<T> {
    fn drop(&mut self) {
        match self.handle_type {
            HandleType::Strong(ref sender) => {
                sender.send(RefChange::Decrement(self.id));
            }
            HandleType::Weak => {}
        }
    }
}

impl<T: Asset> From<Handle<T>> for HandleId {
    fn from(value: Handle<T>) -> Self {
        value.id
    }
}

impl<T: Asset> Handle<T> {
    pub fn strong(id: HandleId, ref_change_sender: Sender<RefChange>) -> Self {
        ref_change_sender.send(RefChange::Increment(id)).unwrap();
        Self {
            id,
            handle_type: HandleType::Strong(ref_change_sender),
            marker: PhantomData::default(),
        }
    }

    pub fn weak(id: HandleId) -> Self {
        Self {
            id,
            handle_type: HandleType::Weak,
            marker:  PhantomData::default(),
        }
    }

    pub fn is_weak(&self) -> bool { matches!(self.handle_type, HandleType::Weak) }

    pub fn is_strong(&self) -> bool { matches!(self.handle_type, HandleType::Strong(_)) }
}


