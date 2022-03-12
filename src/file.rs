use crate::error;

mod data;
pub use data::{Decompressor, Reader, ReaderBuilder};

pub struct File<M = crate::metadata::Full> {
    pub meta: M,
    pub header: FileHeader,
}

pub struct FileHeader {
    start: u64,
    len: u64,
    method: zip_format::CompressionMethod,
    disk_id: u16,
}
impl FileHeader {
    pub(crate) fn new(
        start: u64,
        len: u64,
        method: zip_format::CompressionMethod,
        disk_id: u16,
    ) -> Self {
        Self {
            start,
            len,
            method,
            disk_id,
        }
    }
}

impl<M> File<M> {
    pub fn in_disk<D>(
        self,
        disk: crate::Persisted<crate::Footer, D>,
    ) -> Result<crate::Persisted<File<M>, D>, error::DiskMismatch> {
        (self.header.disk_id == disk.structure.disk_id())
            .then(move || disk.map(move |_| self))
            .ok_or(error::DiskMismatch(()))
    }
}
impl<D, M> crate::Persisted<File<M>, D> {
    pub fn reader(
        self,
        decompressor: &mut Decompressor,
    ) -> Result<crate::Persisted<ReaderBuilder<'_>, D>, error::MethodNotSupported> {
        Ok(crate::Persisted {
            disk: self.disk,
            structure: ReaderBuilder::new(self.structure.header, decompressor)?,
        })
    }
}

impl<M> core::ops::Deref for File<M> {
    type Target = M;
    fn deref(&self) -> &Self::Target {
        &self.meta
    }
}