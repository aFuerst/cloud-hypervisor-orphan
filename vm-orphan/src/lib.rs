use vm_migration::{Snapshottable, Snapshot};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum OrphanableError {
    #[error("Failed to orphan component: {0}")]
    Orphan(#[source] anyhow::Error),

    #[error("Failed to adopt component: {0}")]
    Adopt(#[source] anyhow::Error),
}

pub trait Orphanable : Snapshottable {
    fn orphan(&mut self)  -> std::result::Result<Snapshot, OrphanableError>;

    fn adopt(&mut self)  -> std::result::Result<(), OrphanableError>;
}