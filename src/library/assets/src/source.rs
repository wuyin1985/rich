use std::collections::HashSet;
use crate::asset::LabelId;

pub enum SourceLoadState
{
    None,
    Loading,
    Err,
    Complete,
}


pub struct SourceInfo {
    pub committed_assets:HashSet<LabelId>,
    pub version: usize,
}