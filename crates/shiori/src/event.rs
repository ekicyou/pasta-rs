use shiori3::{ApiResult, ShioriRequestArgs};
use std::path::PathBuf;
use std::sync::mpsc::SyncSender;

pub enum GhostEvent {
    ShioriRequest(ShioriRequestArgs, SyncSender<ApiResult<String>>),
    ShioriNotify(ShioriRequestArgs),
    Load(usize, PathBuf),
    Unload(SyncSender<()>),
}
