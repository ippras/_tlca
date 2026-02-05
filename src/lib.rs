#![feature(custom_inner_attributes)]
#![feature(debug_closure_helpers)]
#![feature(decl_macro)]
#![feature(if_let_guard)]

pub use self::app::App;

mod app;
mod r#const;
mod export;
mod localization;
mod macros;
mod presets;
mod utils;

#[cfg(test)]
mod test {
    use std::{fs::File, io::Write, path::Path};

    use crate::{
        r#const::{AREA, RETENTION_TIME, VALUE},
        utils::{HashedDataFrame, HashedMetaDataFrame},
    };
    use anyhow::Result;
    use lipid::prelude::*;
    use maplit::btreemap;
    use metadata::{
        AUTHORS, DATE, DESCRIPTION, Metadata, NAME, PARAMETERS, VERSION, polars::MetaDataFrame,
    };
    use polars::prelude::*;
    use ron::{extensions::Extensions, ser::PrettyConfig};

    #[test]
    fn test() {}

    #[test]
    fn create_new() -> Result<()> {
        let meta = Metadata(btreemap! {
            AUTHORS.to_owned() => "Giorgi Vladimirovich Kazakov;Roman Alexandrovich Sidorov".to_owned(),
            DATE.to_owned() => "2026-01-19".to_owned(),
            DESCRIPTION.to_owned() => "К-2776\n#1045".to_owned(),
            NAME.to_owned() => "К-2776".to_owned(),
            PARAMETERS.to_owned() => "Experimental".to_owned(),
            VERSION.to_owned() => "0.0.0".to_owned(),
        });

        // | Компонент | Время (мин) | Площадь (мВ*с) | Площадь (%) |
        // | --------- | ----------- | -------------- | ----------- |
        // | POP       | 8.809       | 9.055          | 0.393       |
        // | PPL       | 9.097       | 13.937         | 0.604       |
        // | POS       | 10.199      | 14.811         | 0.642       |
        // | POO       | 10.597      | 26.786         | 1.161       |
        // | PLS       | 10.463      | 106.701        | 4.627       |
        // | PLO       | 10.863      | 182.773        | 7.925       |
        // | PLL       | 11.297      | 125.682        | 5.450       |
        // | SOS       | 0.000       | 0.000          | 0.000       |
        // | SOO       | 12.403      | 77.425         | 3.357       |
        // | SOL       | 12.767      | 273.459        | 11.858      |
        // | OOO       | 12.946      | 130.810        | 5.672       |
        // | SLL       | 13.496      | 97.796         | 4.241       |
        // | OOL       | 13.343      | 522.730        | 22.667      |
        // | LLO       | 13.960      | 528.024        | 22.896      |
        // | LLL       | 14.568      | 196.152        | 8.506       |
        let data = df! {
            LABEL => df! {
                STEREOSPECIFIC_NUMBERS1 => [
                    "P",
                    "P",
                    "P",
                    "P",
                    "P",
                    "P",
                    "P",
                    "S",
                    "S",
                    "S",
                    "O",
                    "S",
                    "O",
                    "L",
                    "L",
                ],
                STEREOSPECIFIC_NUMBERS2 => [
                    "O",
                    "P",
                    "O",
                    "O",
                    "L",
                    "L",
                    "L",
                    "O",
                    "O",
                    "O",
                    "O",
                    "L",
                    "O",
                    "L",
                    "L",
                ],
                STEREOSPECIFIC_NUMBERS3 => [
                    "P",
                    "L",
                    "S",
                    "O",
                    "S",
                    "O",
                    "L",
                    "S",
                    "O",
                    "L",
                    "O",
                    "L",
                    "L",
                    "O",
                    "L",
                ],
            }?.into_struct(PlSmallStr::EMPTY),
            TRIACYLGLYCEROL => df! {
                STEREOSPECIFIC_NUMBERS1 => [
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                ],
                STEREOSPECIFIC_NUMBERS2 => [
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                ],
                STEREOSPECIFIC_NUMBERS3 => [
                    fatty_acid!(C16 {})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                ],
            }?.into_struct(PlSmallStr::EMPTY),
            RETENTION_TIME => [
                8787*60,
                9081*60,
                10181*60,
                10575*60,
                10443*60,
                10848*60,
                11299*60,
                0000*60,
                12374*60,
                12744*60,
                12955*60,
                0000*60,
                0000*60,
                13558*60,
                13337*60,
                13985*60,
                14631*60,
            ],
            AREA => [
                0.270/100.0,
                1.294/100.0,
                0.207/100.0,
                1.380/100.0,
                1.706/100.0,
                7.364/100.0,
                12.132/100.0,
                0.000/100.0,
                0.514/100.0,
                3.003/100.0,
                2.715/100.0,
                0.000/100.0,
                0.000/100.0,
                4.939/100.0,
                11.228/100.0,
                27.916/100.0,
                25.332/100.0,
            ],
        }?
        .lazy()
        .with_columns([
            concat_arr(vec![
                col(RETENTION_TIME).cast(DataType::Duration(TimeUnit::Milliseconds)),
            ])?,
            concat_arr(vec![col(AREA)])?,
        ])
        .collect()?;
        println!("data_frame: {data}");
        let path = Path::new("_output")
            .join(meta.format(".").to_string())
            .with_added_extension("tag.utca.ron");
        let mut file = File::create(&path)?;
        let frame = HashedMetaDataFrame::new(meta, HashedDataFrame::new(data)?);
        let serialized = ron::ser::to_string_pretty(
            &frame,
            PrettyConfig::new().extensions(Extensions::UNWRAP_NEWTYPES),
        )?;
        file.write_all(serialized.as_bytes())?;
        // // MetaDataFrame::new(meta, &mut data).write_parquet(file)?;
        Ok(())
    }
}
