use anyhow::bail;
use calc_rs::solve_func;
use plotlib::repr::Plot;
use plotlib::style::{LineJoin, LineStyle};
use plotlib::view::ContinuousView;
use rayon::prelude::*;

use super::Function;

#[derive(Debug, Clone)]
pub struct PlotConfig {
    /// the function to graph, e.g: "f(x) = 3x^2 + 5x -4".
    pub func: String,
    /// the RGB color in HEX, e.g: "#00EEEE" or "#EEaa22".
    pub color: String,
    /// x and y values combined.
    pub points: Vec<(f64, f64)>,
}

impl From<&Function> for PlotConfig {
    fn from(value: &Function) -> Self {
        PlotConfig {
            func: value.func.clone(),
            color: value.color.clone().unwrap_or("#FFFFFF".to_string()),
            points: Vec::new(),
        }
    }
}

impl PlotConfig {
    fn solve_for(mut self, min: i64, max: i64) -> anyhow::Result<Self> {
        let Ok(res) = solve_func(&self.func, min, max) else {
            bail!("could not solve function")
        };

        self.points = res
            .1
             .0
            .iter()
            .zip(res.1 .1)
            .filter_map(|(x, y)| y.map(|y_val| (*x as f64, y_val)))
            .collect();

        Ok(self)
    }
}

pub fn mk_plot(funcs: Vec<PlotConfig>, min: i64, max: i64) -> anyhow::Result<ContinuousView> {
    let plots: Vec<Plot> = funcs
        .par_iter()
        .map(|conf| conf.clone().solve_for(min, max))
        .collect::<anyhow::Result<Vec<PlotConfig>>>()?
        .into_par_iter()
        .map(|conf| {
            Plot::new(conf.points).line_style(
                LineStyle::new()
                    .linejoin(LineJoin::Round)
                    .colour(conf.color),
            )
        })
        .collect();

    let mut graph = ContinuousView::new();

    for plot in plots {
        graph = graph.add(plot);
    }

    Ok(graph)
}
