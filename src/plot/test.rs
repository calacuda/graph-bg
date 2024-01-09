use super::mk_overlayed_image;
use super::plot::{mk_plot, PlotConfig};
use anyhow::bail;
use plotlib::page::Page;
use std::fs;

#[test]
fn main() -> anyhow::Result<()> {
    // prepare for graph generation
    let confs = vec![
        PlotConfig {
            func: "f(x) = 2x^2".to_string(),
            color: "#00EEEE".to_string(),
            points: Vec::new(),
        },
        PlotConfig {
            func: "g(x) = 4x^2 - 10".to_string(),
            color: "#EEaa22".to_string(),
            points: Vec::new(),
        },
    ];
    let mut base_image = fs::read("./test-data/test-image.png")?;
    // let base = image::load_from_memory(&base_image)?;
    let (min, max) = (-10, 10);

    // make graph
    let Ok(png_graph_data) = mk_overlayed_image(&mut base_image, confs, min, max) else {
        bail!("failed to make overlay")
    };

    fs::write("./test-data/graph.cargo.png", png_graph_data)?;

    Ok(())
}

#[test]
fn plot_test() -> anyhow::Result<()> {
    let confs = vec![
        PlotConfig {
            func: "f(x) = 2x^2".to_string(),
            color: "#00EEEE".to_string(),
            points: Vec::new(),
        },
        PlotConfig {
            func: "g(x) = 4x^2 - 4".to_string(),
            color: "#EEaa22".to_string(),
            points: Vec::new(),
        },
    ];

    let plot = mk_plot(confs, 0, 9)?;

    Page::single(&plot).save("plot.svg").unwrap();

    Ok(())
}
