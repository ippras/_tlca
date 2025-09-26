use crate::utils::{HashedDataFrame, HashedMetaDataFrame};
use metadata::MetaDataFrame;
use std::{io::Cursor, sync::LazyLock};

macro preset($name:literal) {
    LazyLock::new(|| {
        let bytes = include_bytes!($name);
        let MetaDataFrame { meta, data } =
            MetaDataFrame::read_parquet(Cursor::new(bytes)).expect(concat!("preset ", $name));
        MetaDataFrame {
            meta,
            data: HashedDataFrame::new(data).unwrap(),
        }
    })
}

// IPPRAS
#[rustfmt::skip]
pub(crate) mod ippras {
    use super::*;

    pub(crate) static C108_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-108(-N).2025-04-23.tag.utca.parquet");
    pub(crate) static C1210_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-1210(-N).2025-04-24.tag.utca.parquet");
    pub(crate) static C1540_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-1540(-N).2025-04-24.tag.utca.parquet");
    pub(crate) static H626_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/H-626(-N).2025-04-24.tag.utca.parquet");
    pub(crate) static P519_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/P-519(-N).2025-04-23.tag.utca.parquet");

}

// Third party

// [Sidorov2014](https://doi.org/10.1007/s11746-014-2553-8)
#[rustfmt::skip]
pub(crate) mod sidorov2014 {
    use super::*;

    pub(crate) static EUONYMUS_ALATUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Alatus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_BUNGEANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Bungeanus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_EUROPAEUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Europaeus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_HAMILTONIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Hamiltonianus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_LATIFOLIUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Latifolius.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_MACROPTERUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Macropterus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_MAXIMOWICZIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Maximowiczianus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_PAUCIFLORUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Pauciflorus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_PHELLOMANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Phellomanus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_SACHALINENSIS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sachalinensis.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_SACROSANCTUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sacrosanctus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_SEMIEXSERTUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Semiexsertus.2014-06-19.tag.utca.parquet");
    pub(crate) static EUONYMUS_SIEBOLDIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sieboldianus.2014-06-19.tag.utca.parquet");
}

// [Martínez-Force2004](https://doi.org/10.1016/j.ab.2004.07.019)
#[rustfmt::skip]
pub(crate) mod martínez_force2004 {
    use super::*;

    pub(crate) static HAZELNUT: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Hazelnut.2025-08-19.tag.utca.parquet");
    pub(crate) static OLIVE: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Olive.2025-08-19.tag.utca.parquet");
    pub(crate) static RICE: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Rice.2025-08-19.tag.utca.parquet");
    pub(crate) static SOYBEAN: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Soybean.2025-08-19.tag.utca.parquet");
    pub(crate) static SUNFLOWER_CAS3: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Sunflower CAS-3.2025-08-19.tag.utca.parquet");
    pub(crate) static SUNFLOWER_RHA274: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Sunflower RHA-274.2025-08-19.tag.utca.parquet");
    pub(crate) static WALNUT: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Walnut.2025-08-19.tag.utca.parquet");
}
