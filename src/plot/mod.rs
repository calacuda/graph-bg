use anyhow::bail;
use image::overlay_image;
use plot::{mk_plot, PlotConfig};
use plotlib::page::Page;
use serde::{Deserialize, Serialize};

#[cfg(test)]
mod test;

pub mod image;
pub mod plot;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Function {
    /// the function to graph, e.g: "f(x) = 2x^2 + 5x -4".
    pub func: String,
    /// the RGB color in HEX, e.g: "#01EEEE" or "#EEaa22".
    pub color: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SolveQuery {
    /// The base image file data encoded as Base63. only excepts PNGs.
    // pub image: std::fs::File, // Vec<u8>,
    /// A list of functions to solve and colors to use for their graph.
    pub funcs: Vec<Function>, // Vec<Function>,
    // pub funcs: String,
    /// She inclusive range to graph the functions over. Should be in the format (min, max).
    pub min: i64,
    pub max: i64,
}

/// standard entry point to the library.
/// It takes:
/// - data from a .png as a Vec of u8's (`base_image_data`)
/// - a vec of fuctions configs which describe the function to solve and what color its line
/// should be (`confs`)
/// - a min and max value for x, these form a range between which the functions will be graphed
/// (`min` & `max`).
pub fn mk_overlayed_image(
    base_image_data: &mut [u8],
    confs: Vec<PlotConfig>,
    min: i64,
    max: i64,
) -> Result<String, String> {
    use base64::{engine::general_purpose, Engine as _};

    // let image: Vec<u8> =g general_purpose::STANDARD_NO_PAD.decode(&image_b64)
    match overlay(base_image_data, confs, min, max) {
        Ok(data) => Ok(general_purpose::STANDARD.encode(&data)),
        Err(e) => Err(e.to_string()),
    }
}

fn overlay(
    base_image_data: &mut [u8],
    confs: Vec<PlotConfig>,
    min: i64,
    max: i64,
) -> anyhow::Result<Vec<u8>> {
    // TODO: check for valid png here.
    let img = ::image::load_from_memory(base_image_data)?;
    // println!("loaded image");
    if resvg::tiny_skia::PixmapMut::from_bytes(
        &mut img.as_bytes().to_owned(),
        img.width(),
        img.height(),
    )
    .is_none()
    {
        bail!("file needs to be a png encode with RGBA");
    }

    // make plot
    let Ok(doc) = Page::single(&mk_plot(confs, min, max)?).to_svg() else {
        bail!("failed to convert plot to svg")
    };

    // overlay plot over image
    // TODO: make this take the img variable from above.
    overlay_image(base_image_data, &doc.to_string())
}
