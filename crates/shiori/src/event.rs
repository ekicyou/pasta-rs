use shiori3::{ApiResult, GCowStr};
use std::path::PathBuf;
use std::sync::mpsc::SyncSender;

pub enum GhostEvent {
    ShioriRequest(GCowStr, SyncSender<ApiResult<String>>),
    ShioriNotify(GCowStr),
    Load(usize, PathBuf),
    Unload(SyncSender<()>),
}
