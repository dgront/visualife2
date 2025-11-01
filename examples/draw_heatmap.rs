use rand::Rng;
use clap::{Parser, ArgAction};

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

    /// Columns for row_index, col_index, value (1-based indices); if not specified, first three columns are used
    #[arg(short = 'c', long = "columns", value_names = ["row", "col", "val"], num_args = 3)]
    columns: Option<Vec<usize>>,

    /// Columns for row_labels and col_labels (1-based indices)
    #[arg(short = 'l', long = "labels", value_names = ["row_labels", "col_labels"], num_args = 2)]
    labels: Option<Vec<usize>>,

    /// Input is a single-column matrix in row-wise order (square matrix only)
    #[arg(short = 's', long = "single-column")]
    single_column: Option<usize>,

    /// Input file has also explicit row/column indexes (optional)
    #[arg(long)]
    indexes: Option<Vec<usize>>,

    /// Make matrix symmetric (set both i,j and j,i)
    #[arg(short = 'm', long = "make-symmetric", action = ArgAction::SetTrue)]
    make_symmetric: bool,

    /// skip the header line; note that comment lines starting with '#' are always skipped
    #[arg(long = "skip-header", action = ArgAction::SetTrue)]
    skip_header: bool,

    /// Output SVG file (default: heatmap.svg)
    #[arg(short = 'o', long = "output", default_value = "heatmap.svg")]
    output: String,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let x_margin = 100.0;
    let y_margin = 50.0;
    let box_size = 20.0;
    let mut data_source: DataMatrixBuilder = DataMatrixBuilder::new();
    let dm = match &args.input {
        Some(filename) => {
            if let Some(col_idx) = args.single_column {
                data_source = data_source.data_column(col_idx)
            }
            if let Some(row_col_val) = args.columns {
                data_source = data_source.index_columns(row_col_val[0], row_col_val[1]).data_column(row_col_val[2]);
            }
            if let Some(row_col_labels) = args.labels {
                data_source = data_source.label_columns(row_col_labels[0], row_col_labels[1]);
            }
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
            let mut rng = rand::thread_rng();
            let data: Vec<f64> = (0..n*n).map(|_| rng.random::<f64>()).collect();
            data_source.from_data(&data)
        }
    }?;
    eprintln!("{:?}",dm.get(0, 1));
    eprintln!("{:?}",dm.get(1, 2));

    let n = dm.ncols();
    let draw_width = box_size * n as f32 + 2.0 * x_margin;
    let draw_height = box_size * n as f32 + 2.0 * y_margin;
    let mut drawing = SvgDrawing::new(draw_width, draw_height);

    let mut map = Heatmap::from_datamatrix("heatmap", box_size, box_size, &dm);
    map.offset_x = x_margin;
    map.offset_y = y_margin;
    drawing.add_element(map.create_element());
    drawing.save_svg(&args.output)?;
    eprintln!("Saved {}", args.output);
    Ok(())
}
