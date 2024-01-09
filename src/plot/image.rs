use anyhow::bail;
use resvg::usvg::fontdb::Database;
use resvg::usvg::TreeParsing;
use resvg::usvg::{self, TreeTextToPath};

pub fn overlay_image(base_image: &mut [u8], plot_svg: &str) -> anyhow::Result<Vec<u8>> {
    let base = image::load_from_memory(base_image)?;
    let mut base_bytes = base.as_bytes().to_owned();

    let (plot_w, plot_h) = (base.width() as f32, base.height() as f32);
    let Some(mut pixmap) =
        // resvg::tiny_skia::PixmapMut::from_bytes(&mut base_bytes, base.width(), base.height())
        resvg::tiny_skia::PixmapMut::from_bytes(&mut base_bytes, base.width(), base.height())
    else {
        bail!("could not read base image");
    };
    let Some(size) = resvg::usvg::Size::from_wh(plot_w, plot_h) else {
        bail!("unknown Error")
    };
    let options = usvg::Options::default();
    let mut tree = usvg::Tree::from_str(plot_svg, &options)?;
    tree.size = size;
    let mut font = Database::new();
    font.load_system_fonts();
    tree.convert_text(&font);
    resvg::Tree::from_usvg(&tree).render(resvg::tiny_skia::Transform::identity(), &mut pixmap);

    Ok(pixmap.to_owned().encode_png()?)
}
