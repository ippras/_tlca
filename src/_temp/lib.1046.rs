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
            DESCRIPTION.to_owned() => "Бузулук\n#1046".to_owned(),
            NAME.to_owned() => "Бузулук".to_owned(),
            PARAMETERS.to_owned() => "Experimental".to_owned(),
            VERSION.to_owned() => "0.0.0".to_owned(),
        });

        // | Компонент | Время (мин) | Площадь (мВ*с) | Площадь (%) |
        // | --------- | ----------- | -------------- | ----------- |
        // | P       | 8.787       | 9.043          | 0.270       |
        // | L       | 9.081       | 43.256         | 1.294       |
        // | S       | 10.181      | 6.936          | 0.207       |
        // | O       | 10.575      | 46.142         | 1.380       |
        // | S       | 10.443      | 57.043         | 1.706       |
        // | O       | 10.848      | 246.266        | 7.364       |
        // | L       | 11.299      | 405.689        | 12.132      |
        // | S       | 0.000       | 0.000          | 0.000       |
        // | O       | 12.374      | 17.188         | 0.514       |
        // | L       | 12.744      | 100.409        | 3.003       |
        // | O       | 12.955      | 90.780         | 2.715       |
        // | L       | 0.000       | 0.000          | 0.000       |
        // | L       | 0.000       | 0.000          | 0.000       |
        // | L       | 13.558      | 165.175        | 4.939       |
        // | L       | 13.337      | 375.469        | 11.228      |
        // | O       | 13.985      | 933.516        | 27.916      |
        // | L       | 14.631      | 847.096        | 25.332      |
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
"Stearic",
"Oleic",
"Oleic",
"Oleic",
"Stearic",
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
"Oleic",
"Linoleic",
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
"Linoleic",
"Oleic",
"Linoleic",
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
