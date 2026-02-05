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
        r#const::VALUE,
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
        // №1041 2026-01-16 14:06:29
        let meta = Metadata(btreemap! {
            AUTHORS.to_owned() => "Giorgi Vladimirovich Kazakov;Roman Alexandrovich Sidorov".to_owned(),
            DATE.to_owned() => "2026-01-16".to_owned(),
            DESCRIPTION.to_owned() => "К-2233, Прогресс, Россия\n#1041".to_owned(),
            NAME.to_owned() => "К-2233".to_owned(),
            PARAMETERS.to_owned() => "Experimental".to_owned(),
            VERSION.to_owned() => "0.0.0".to_owned(),
        });

        // | Компонент | Время (мин) | Площадь (мВ*с) | Площадь (%) |
        // | --------- | ----------- | -------------- | ----------- |
        // | POP       | 8.708       | 16.778         | 0.491       |
        // | PPL       | 8.999       | 23.194         | 0.678       |
        // | POS       | 10.098      | 30.371         | 0.888       |
        // | POO       | 10.363      | 183.131        | 5.356       |
        // | PLS       | 10.476      | 39.874         | 1.166       |
        // | PLO       | 10.769      | 267.971        | 7.837       |
        // | PLL       | 11.195      | 165.303        | 4.835       |
        // | SOS       | 12.021      | 5.328          | 0.156       |
        // | SOO       | 12.337      | 170.878        | 4.998       |
        // | OOO       | 12.719      | 489.728        | 14.323      |
        // | SOL       | 12.869      | 193.800        | 5.668       |
        // | OOL       | 13.272      | 829.611        | 24.264      |
        // | SLL       | 13.463      | 86.986         | 2.544       |
        // | LLO       | 13.892      | 678.107        | 19.833      |
        // | LLL       | 14.475      | 238.033        | 6.962       |
        let data = df! {
            LABEL => df! {
                STEREOSPECIFIC_NUMBERS1 => [
                    "Palmitic",
                    "Palmitic",
                    "Palmitic",
                    "Palmitic",
                    "Palmitic",
                    "Palmitic",
                    "Palmitic",
                    "Stearic",
                    "Stearic",
                    "Oleic",
                    "Stearic",
                    "Oleic",
                    "Stearic",
                    "Linoleic",
                    "Linoleic",
                ],
                STEREOSPECIFIC_NUMBERS2 => [
                    "Oleic",
                    "Palmitic",
                    "Oleic",
                    "Oleic",
                    "Linoleic",
                    "Linoleic",
                    "Linoleic",
                    "Oleic",
                    "Oleic",
                    "Oleic",
                    "Oleic",
                    "Oleic",
                    "Linoleic",
                    "Linoleic",
                    "Linoleic",
                ],
                STEREOSPECIFIC_NUMBERS3 => [
                    "Palmitic",
                    "Linoleic",
                    "Stearic",
                    "Oleic",
                    "Stearic",
                    "Oleic",
                    "Linoleic",
                    "Stearic",
                    "Oleic",
                    "Oleic",
                    "Linoleic",
                    "Linoleic",
                    "Linoleic",
                    "Oleic",
                    "Linoleic",
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
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {})?,
                    fatty_acid!(C18 {9 => C})?,
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
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                    fatty_acid!(C18 {9 => C})?,
                    fatty_acid!(C18 {9 => C, 12 => C})?,
                ],
            }?.into_struct(PlSmallStr::EMPTY),
            VALUE => [
                0.491/100.0,
                0.678/100.0,
                0.888/100.0,
                5.356/100.0,
                1.166/100.0,
                7.837/100.0,
                4.835/100.0,
                0.156/100.0,
                4.998/100.0,
                14.323/100.0,
                5.668/100.0,
                24.264/100.0,
                2.544/100.0,
                19.833/100.0,
                6.962/100.0,
            ],
        }?
        .lazy()
        .with_column(concat_arr(vec![col(VALUE)])?)
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
