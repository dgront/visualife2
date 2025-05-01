use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::fs;
use clap::Parser;

use datamatrix::{DataMatrixBuilder};

use visualife::{SvgDrawing};
use visualife::heatmap::Heatmap;
use visualife::styling::{ColorMap};
use visualife::styling::palettes::RED_BLUE;

/// Command line app for visualizing matrix as heatmaps
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Input file with matrix data (optional)
    #[arg(short = 'i', long = "input")]
    input: Option<String>,

    /// Columns for col_label, row_label, value (1-based indices); if not specified, first three columns are used
    #[arg(short = 'c', long = "columns", value_names = ["col", "row", "val"], num_args = 3)]
    columns: Option<Vec<usize>>,

    /// Input is a single-column matrix in row-wise order (square matrix only)
    #[arg(short = 's', long = "single-column")]
    single_column: Option<usize>,

    /// Input file has also explicit row/column indexes (optional)
    #[arg(long)]
    indexes: Option<Vec<usize>>,

    /// Make matrix symmetric (set both i,j and j,i)
    #[arg(short = 'm', long = "make-symmetric")]
    make_symmetric: bool,

    /// Output SVG file (default: heatmap.svg)
    #[arg(short = 'o', long = "output", default_value = "heatmap.svg")]
    output: String,
}


fn read_column(fname: &str, which_column: usize) -> Result<Vec<f64>, String> {
    let reader = BufReader::new(File::open(fname).map_err(|e| e.to_string())?);

    reader
        .lines()
        .enumerate()
        .map(|(i, line)| {
            let line = line.map_err(|e| e.to_string())?;
            line.split_whitespace()
                .nth(which_column)
                .ok_or_else(|| format!("Missing column {} on line {}", which_column, i + 1))?
                .parse::<f64>()
                .map_err(|e| format!("Parse error on line {}: {}", i + 1, e))
        })
        .collect()
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let margin = 20.0;
    let box_size = 5.0;
    let mut data_source: DataMatrixBuilder = DataMatrixBuilder::new();
    let mut data: Option<Vec<f64>> = None;
    match &args.input {
        Some(filename) => {
            if let Some(col_idx) = args.single_column {
                data = Some(read_column(filename, col_idx)?);
            }
            if let Some(row_col_val) = args.columns {
                data_source = data_source.index_columns(row_col_val[0], row_col_val[1]).data_column(row_col_val[2]);
            }
            if let Some(row_col) = args.indexes {
                data_source = data_source.index_columns(row_col[0], row_col[1]);
            }
        }
        None => {
            let n = 30;
            let mut rng = rand::thread_rng();
            data = Some((0..n*n).map(|_| rng.random::<f64>()).collect());
        }
    }

    let dm = match data {
        Some(data) => data_source.from_data(&data),
        None => data_source.from_file(&args.input.unwrap())
    }.map_err(|e| e.to_string())?;

    let n = dm.ncols();
    let draw_width = box_size * n as f32 + 2.0 * margin;
    let cmap = ColorMap::from_range(&RED_BLUE, 0.0, 1.0)?;
    let drawing = SvgDrawing::new(draw_width, draw_width);

    let map = Heatmap::from_data(drawing, "heatmap", box_size, box_size, dm.data().clone(), &cmap);
    fs::write(&args.output, &map.to_svg()).map_err(|e| e.to_string())?;

    eprintln!("Saved {}", args.output);
    Ok(())
}
