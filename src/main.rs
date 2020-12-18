use anyhow::Context;
use graphicsmagick::{initialize, types::FilterTypes, wand::MagickWand};
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // This function should be invoked in the primary (original) thread
    // of the application's process, and before starting any OpenMP
    // threads, as part of program initialization.
    initialize();

    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("data")
        .join("b93b6081f3b4082b138f82ffb5a743ac.gif");
    let path = path.to_str().context("get image path failed")?;

    // let mut mw1 = MagickWand::new();
    // dbg!(mw1.get_image_profile(&path)?);

    let mut mw = MagickWand::new();
    let wand = mw.read_image(path)?;
    for idx in 0..wand.get_image_index() {
        wand.set_image_index(idx as i64)?;
        wand.resize_image(200, 200, FilterTypes::CubicFilter, 1.)?;
    }

    wand.write_images("data/output.gif", 4)?;

    Ok(())
}
