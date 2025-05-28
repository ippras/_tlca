use crate::utils::Hashed;
use metadata::MetaDataFrame;
use polars::prelude::*;
use std::{io::Cursor, sync::LazyLock};

macro preset($name:literal) {
    LazyLock::new(|| {
        let bytes = include_bytes!($name);
        Hashed::new(convert(
            MetaDataFrame::read_parquet(Cursor::new(bytes)).expect(concat!("preset ", $name)),
        ))
    })
}

// IPPRAS
pub(crate) mod ippras {
    use super::*;

    pub(crate) static LOBOSPHERA_N_1: LazyLock<Hashed<MetaDataFrame>> =
        preset!("ippras/Lobosphera-N.2025-04-24.0.0.1.utca.ipc");
    // pub(crate) static LOBOSPHERA_N_2: LazyLock<Hashed<MetaDataFrame>> =
    //     preset!("ippras/Lobosphera-N.2025-04-24.0.0.2.utca.ipc");
    // pub(crate) static LOBOSPHERA_N_3: LazyLock<Hashed<MetaDataFrame>> =
    //     preset!("ippras/Lobosphera-N.2025-04-24.0.0.3.utca.ipc");

    pub(crate) static _519_N: LazyLock<Hashed<MetaDataFrame>> =
        preset!("ippras/519-N.2025-04-23.0.0.1.utca.ipc");
    pub(crate) static C108_N: LazyLock<Hashed<MetaDataFrame>> =
        preset!("ippras/C108-N.2025-04-23.0.0.1.utca.ipc");
    pub(crate) static C1210_N: LazyLock<Hashed<MetaDataFrame>> =
        preset!("ippras/C1210-N.2025-04-24.0.0.1.utca.ipc");
    pub(crate) static H626_N: LazyLock<Hashed<MetaDataFrame>> =
        preset!("ippras/H626-N.2025-04-24.utca.ipc");
}

use lipid::{
    polars::expr::ExprExt,
    prelude::{DC, FATTY_ACID_DATA_TYPE, FattyAcidChunked, S},
};
use polars::chunked_array::builder::AnonymousOwnedListBuilder;

fn convert(frame: MetaDataFrame) -> MetaDataFrame {
    let lazy_frame = frame
        .data
        .clone()
        .lazy()
        .select([
            col("Label"),
            col("FattyAcid")
                .triacylglycerol()
                .map_expr(|expr| {
                    expr.apply(
                        to_fatty_acid,
                        GetOutput::from_type(DataType::List(Box::new(
                            FATTY_ACID_DATA_TYPE.clone(),
                        ))),
                    )
                })
                .alias("Triacylglycerol"),
            col("Value"),
        ])
        .select([
            as_struct(vec![
                as_struct(vec![
                    col("Label").struct_().field_by_index(0).alias("Label"),
                    col("Triacylglycerol")
                        .struct_()
                        .field_by_index(0)
                        .alias("FattyAcid"),
                ])
                .alias("StereospecificNumber1"),
                as_struct(vec![
                    col("Label").struct_().field_by_index(1).alias("Label"),
                    col("Triacylglycerol")
                        .struct_()
                        .field_by_index(1)
                        .alias("FattyAcid"),
                ])
                .alias("StereospecificNumber2"),
                as_struct(vec![
                    col("Label").struct_().field_by_index(2).alias("Label"),
                    col("Triacylglycerol")
                        .struct_()
                        .field_by_index(2)
                        .alias("FattyAcid"),
                ])
                .alias("StereospecificNumber3"),
            ])
            .alias("Triacylglycerol"),
            // col("Triacylglycerol"),
            col("Value"),
        ]);
    println!("lazy_frame: {}", lazy_frame.clone().collect().unwrap());
    MetaDataFrame::new(frame.meta, lazy_frame.collect().expect("Preset"))
}

fn to_fatty_acid(column: Column) -> PolarsResult<Option<Column>> {
    let series = column.as_materialized_series();
    let carbons = series.struct_()?.field_by_name("Carbons")?;
    let unsaturated = series.struct_()?.field_by_name("Unsaturated")?;
    let mut list = AnonymousOwnedListBuilder::new(
        PlSmallStr::EMPTY,
        column.len(),
        Some(FATTY_ACID_DATA_TYPE.clone()),
    );
    for (carbons, unsaturated) in carbons
        .u8()?
        .into_no_null_iter()
        .zip(unsaturated.list()?.into_no_null_iter())
    {
        let mut bounds = Vec::new();
        for index in 1..carbons {
            let unsaturated_index = unsaturated.struct_()?.field_by_name("Index")?;
            let mask = unsaturated_index.u8()?.equal(index);
            if mask.any() {
                bounds.push(DC);
            } else {
                bounds.push(S);
            }
        }
        let fatty_acid = FattyAcidChunked::try_from(&*bounds)?;
        list.append_series(&fatty_acid.into_struct(PlSmallStr::EMPTY)?.into_series())?;
    }
    Ok(Some(list.finish().into_column()))
}
