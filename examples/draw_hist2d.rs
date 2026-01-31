use clap::Parser;
use clap::ArgAction;
use data_matrix::DataMatrixBuilder;
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use visualife::plots::{linspace_by, Plot};
use visualife::SvgDrawing;

/// Command line app to visualize a 2D histogram as a heatmap
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input file with matrix data (optional)
    #[arg(short = 'i', long = "input file with histogram values: x_bin_from, y_bin_from, hist_value - the CSV format")]
    input: Option<String>,

    /// Make matrix symmetric (set both i,j and j,i)
    #[arg(short = 'm', long = "make-symmetric", action = ArgAction::SetTrue)]
    make_symmetric: bool,

    /// skip the header line; note that comment lines starting with '#' are always skipped
    #[arg(long = "skip-header", action = ArgAction::SetTrue)]
    skip_header: bool,

    /// Output SVG file (default: hist2d.svg)
    #[arg(short = 'o', long = "output", default_value = "hist2d.svg")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let mut data_source: DataMatrixBuilder = DataMatrixBuilder::new();
    let dm = match &args.input {
        Some(filename) => {
            if args.skip_header {
                data_source = data_source.skip_header(true);
            }
            if args.make_symmetric {
                data_source = data_source.symmetric(true);
            }
            data_source.from_file(filename)
        }
        None => {
            let n = 30;
            let mut rng = StdRng::seed_from_u64(0);
            let labels: Vec<String> = (0..n).into_iter().map(|i| i.to_string()).collect();
            let data: Vec<f64> = (0..n * n).map(|v| v as f64/100.0 + rng.random::<f64>()).collect();
            data_source.labels(labels).from_data(&data)
        }
    }?;

    let mut x: Vec<f32> = dm.col_labels().iter().filter_map(|v| v.parse::<f32>().ok()).collect();
    // --- we need one extra point to define axis - the end of the data range
    // --- here we assume each box of the heatmap data has the same size
    x.push(x[1] - x[0] + x[x.len() - 1]);
    let data = dm.data();
    let width = 700.0;
    let m = 75.0;
    let mut plot = Plot::rectangular("hist", (0.0 + m, width - m, 0.0 + m, width - m));
    plot.set_nticks(5);
    plot.heatmap(&x, &x, data.clone())?;
    let mut drawing = SvgDrawing::new(width, width);
    drawing.add_element(plot.create_element());
    drawing.save_svg(&args.output)?;

    Ok(())
}