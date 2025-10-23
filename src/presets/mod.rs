use crate::utils::{HashedDataFrame, HashedMetaDataFrame};
use anyhow::Result;
use metadata::polars::MetaDataFrame;
use std::sync::LazyLock;

macro preset($name:literal) {
    LazyLock::new(|| parse(include_bytes!($name)).expect(concat!("preset ", $name)))
}

fn parse(bytes: &[u8]) -> Result<HashedMetaDataFrame> {
    let frame = ron::de::from_bytes::<MetaDataFrame>(bytes)?;
    Ok(MetaDataFrame {
        meta: frame.meta,
        data: HashedDataFrame::new(frame.data).unwrap(),
    })
}

// IPPRAS
pub(crate) mod ippras {
    #[rustfmt::skip]
    pub(crate) mod fa {
        use super::super::*;

        pub(crate) static C108_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-108(-N).2025-04-23.fa.utca.ron");
        pub(crate) static C1210_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-1210(-N).2025-04-24.fa.utca.ron");
        pub(crate) static C1540_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-1540(-N).2025-04-24.fa.utca.ron");
        // pub(crate) static H242_CONTROL: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/H-242(Control).2023-10-24.fa.utca.ron");
        pub(crate) static H626_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/H-626(-N).2025-04-24.fa.utca.ron");
        pub(crate) static P519_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/P-519(-N).2025-04-23.fa.utca.ron");
    }

    #[rustfmt::skip]
    pub(crate) mod tag {
        use super::super::*;

        pub(crate) static C108_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-108(-N).2025-04-23.tag.utca.ron");
        pub(crate) static C1210_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-1210(-N).2025-04-24.tag.utca.ron");
        pub(crate) static C1540_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/C-1540(-N).2025-04-24.tag.utca.ron");
        pub(crate) static H626_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/H-626(-N).2025-04-24.tag.utca.ron");
        pub(crate) static P519_N: LazyLock<HashedMetaDataFrame> = preset!("ippras/Microalgae/P-519(-N).2025-04-23.tag.utca.ron");
    }
}

// [Sidorov2014](https://doi.org/10.1007/s11746-014-2553-8)
pub(crate) mod sidorov2014 {
    #[rustfmt::skip]
    pub(crate) mod fa {
        use super::super::*;

        pub(crate) static EUONYMUS_ALATUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Alatus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_BUNGEANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Bungeanus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_EUROPAEUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Europaeus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_HAMILTONIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Hamiltonianus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_LATIFOLIUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Latifolius.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_MACROPTERUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Macropterus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_MAXIMOWICZIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Maximowiczianus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_PAUCIFLORUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Pauciflorus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_PHELLOMANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Phellomanus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_SACHALINENSIS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sachalinensis.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_SACROSANCTUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sacrosanctus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_SEMIEXSERTUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Semiexsertus.2014-06-19.fa.utca.ron");
        pub(crate) static EUONYMUS_SIEBOLDIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sieboldianus.2014-06-19.fa.utca.ron");
    }

    #[rustfmt::skip]
    pub(crate) mod tag {
        use super::super::*;

        // pub(crate) static EUONYMUS_ALATUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Alatus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_BUNGEANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Bungeanus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_EUROPAEUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Europaeus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_HAMILTONIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Hamiltonianus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_LATIFOLIUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Latifolius.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_MACROPTERUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Macropterus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_MAXIMOWICZIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Maximowiczianus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_PAUCIFLORUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Pauciflorus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_PHELLOMANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Phellomanus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_SACHALINENSIS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sachalinensis.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_SACROSANCTUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sacrosanctus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_SEMIEXSERTUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Semiexsertus.2014-06-19.tag.utca.ron");
        // pub(crate) static EUONYMUS_SIEBOLDIANUS: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2014/Euonymus Sieboldianus.2014-06-19.tag.utca.ron");
    }
}

// [Sidorov2014](https://doi.org/10.3390/plants14040612)
pub(crate) mod sidorov2025 {
    #[rustfmt::skip]
    pub(crate) mod fa {
        use super::super::*;

        // [Реакция Фишера-Шпайера](https://doi.org/10.1002/cber.189502803176)
        pub(crate) static LUNARIA_REDIVIVA_FISCHER: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2025/Lunaria Rediviva(Fischer).2024-01-24.fa.utca.ron");
        pub(crate) static LUNARIA_REDIVIVA_SAPONIFICATION: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2025/Lunaria Rediviva(Saponification).2024-01-24.fa.utca.ron");
        pub(crate) static LUNARIA_REDIVIVA_TMSH: LazyLock<HashedMetaDataFrame> = preset!("ippras/Sidorov2025/Lunaria Rediviva(TMSH).2024-01-24.fa.utca.ron");
    }

    #[rustfmt::skip]
    pub(crate) mod tag {
        use super::super::*;

    }
}

// Third party

// [Reske1997](https://doi.org/10.1007/s11746-997-0016-1)
pub(crate) mod reske1997 {
    #[rustfmt::skip]
    pub(crate) mod fa {
        use super::super::*;

        pub(crate) static SUNFLOWER_SEED_COMMODITY: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Reske1997/Sunﬂower Seed (Commodity).1997-08-01.fa.utca.ron");
        pub(crate) static SUNFLOWER_SEED_HIGH_LINOLEIC: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Reske1997/Sunﬂower Seed (High linoleic).1997-08-01.fa.utca.ron");
        pub(crate) static SUNFLOWER_SEED_HIGH_OLEIC: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Reske1997/Sunﬂower Seed (High oleic).1997-08-01.fa.utca.ron");
        pub(crate) static SUNFLOWER_SEED_HIGH_PALMITIC_HIGH_LINOLEIC: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Reske1997/Sunﬂower Seed (High palmitic, high linoleic).1997-08-01.fa.utca.ron");
        pub(crate) static SUNFLOWER_SEED_HIGH_PALMITIC_HIGH_OLEIC: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Reske1997/Sunﬂower Seed (High palmitic, high oleic).1997-08-01.fa.utca.ron");
        pub(crate) static SUNFLOWER_SEED_HIGH_STEARIC_HIGH_OLEIC: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Reske1997/Sunﬂower Seed (High stearic, high oleic).1997-08-01.fa.utca.ron");
    }

    #[rustfmt::skip]
    pub(crate) mod tag {}
}

// [Martínez-Force2004](https://doi.org/10.1016/j.ab.2004.07.019)
pub(crate) mod martínez_force2004 {
    #[rustfmt::skip]
    pub(crate) mod fa {
        use super::super::*;

        pub(crate) static HAZELNUT: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Hazelnut.2025-08-19.fa.utca.ron");
        pub(crate) static OLIVE: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Olive.2025-08-19.fa.utca.ron");
        pub(crate) static RICE: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Rice.2025-08-19.fa.utca.ron");
        pub(crate) static SOYBEAN: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Soybean.2025-08-19.fa.utca.ron");
        pub(crate) static SUNFLOWER_CAS3: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Sunflower CAS-3.2025-08-19.fa.utca.ron");
        pub(crate) static SUNFLOWER_RHA274: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Sunflower RHA-274.2025-08-19.fa.utca.ron");
        pub(crate) static WALNUT: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Walnut.2025-08-19.fa.utca.ron");
    }

    #[rustfmt::skip]
    pub(crate) mod tag {
        use super::super::*;

        // pub(crate) static HAZELNUT: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Hazelnut.2025-08-19.tag.utca.ron");
        // pub(crate) static OLIVE: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Olive.2025-08-19.tag.utca.ron");
        // pub(crate) static RICE: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Rice.2025-08-19.tag.utca.ron");
        // pub(crate) static SOYBEAN: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Soybean.2025-08-19.tag.utca.ron");
        // pub(crate) static SUNFLOWER_CAS3: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Sunflower CAS-3.2025-08-19.tag.utca.ron");
        // pub(crate) static SUNFLOWER_RHA274: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Sunflower RHA-274.2025-08-19.tag.utca.ron");
        // pub(crate) static WALNUT: LazyLock<HashedMetaDataFrame> = preset!("ThirdParty/Martinez-Force2004/Walnut.2025-08-19.tag.utca.ron");
    }
}
