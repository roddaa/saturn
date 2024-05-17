use log::LevelFilter;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Logger {
    pub level: LevelFilter,
    pub file: PathBuf,
    pub rotation: Rotation,
}

#[derive(Debug)]
pub struct Rotation {
    pub size: u64,

}

