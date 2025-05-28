use anyhow::Result;
use metadata::MetaDataFrame;
use std::fs::File;

// let file = File::create(name)?;
// let mut writer = IpcWriter::new(file);
// writer.finish(data_frame)?;
//         self.tree
//             .insert_pane::<VERTICAL>(Pane::new(vec![$frame.clone()]));
//     }
// }

#[cfg(not(target_arch = "wasm32"))]
pub fn save(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
    use polars::{io::SerWriter as _, prelude::IpcWriter};
    use std::sync::Arc;

    let file = File::create(name)?;
    MetaDataFrame::new(frame.meta.clone(), &mut frame.data).write_parquet(file)?;
    // let mut writer = IpcWriter::new(file);
    // writer.set_custom_schema_metadata(Arc::new(frame.meta.clone().into()));
    // writer.finish(&mut frame.data)?;
    // MetaDataFrame::new(frame.meta.clone(), &mut frame.data).write_ipc(file)?;
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub fn save(name: &str, frame: &mut MetaDataFrame) -> Result<()> {
    use anyhow::anyhow;
    use egui_ext::download;

    let mut bytes = Vec::new();
    MetaDataFrame::new(frame.meta.clone(), &mut frame.data).write_ipc(&mut bytes)?;
    download(name, &bytes).map_err(|error| anyhow!(error))
}
