use crate::{
    app::HashedMetaDataFrame,
    utils::{Hashed, hash_data_frame},
};
use metadata::MetaDataFrame;
use std::{io::Cursor, sync::LazyLock};

macro preset($name:literal) {
    LazyLock::new(|| {
        let bytes = include_bytes!($name);
        let MetaDataFrame { meta, mut data } =
            MetaDataFrame::read_parquet(Cursor::new(bytes)).expect(concat!("preset ", $name));
        let hash = hash_data_frame(&mut data).unwrap();
        MetaDataFrame {
            meta,
            data: Hashed { value: data, hash },
        }
    })
}

// IPPRAS
pub(crate) mod ippras {
    use super::*;

    pub(crate) static C108_N: LazyLock<HashedMetaDataFrame> =
        preset!("ippras/C-108(-N).2025-04-23.tlca.parquet");
    pub(crate) static C1210_N: LazyLock<HashedMetaDataFrame> =
        preset!("ippras/C-1210(-N).2025-04-24.tlca.parquet");
    pub(crate) static C1540_N: LazyLock<HashedMetaDataFrame> =
        preset!("ippras/C-1540(-N).2025-04-24.tlca.parquet");
    pub(crate) static H626_N: LazyLock<HashedMetaDataFrame> =
        preset!("ippras/H-626(-N).2025-04-24.tlca.parquet");
    pub(crate) static P519_N: LazyLock<HashedMetaDataFrame> =
        preset!("ippras/P-519(-N).2025-04-23.tlca.parquet");
}
