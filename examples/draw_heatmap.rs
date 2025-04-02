use rand::Rng;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::fs;
use std::collections::HashMap;
use clap::Parser;
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

    /// Columns for i, j, value (1-based indices)
    #[arg(short = 'c', long = "columns", value_names = ["I", "J", "VAL"], num_args = 3)]
    columns: Option<Vec<usize>>,

    /// Input is a single-column matrix in row-wise order (square matrix only)
    #[arg(short = 's', long = "single-column")]
    single_column: bool,

    /// Input file has row/column string labels (e.g., "2H7S:A  1DZ6:B  0.005270")
    #[arg(short = 'a', long = "annotated")]
    annotated: bool,

    /// Make matrix symmetric (set both i,j and j,i)
    #[arg(short = 'm', long = "make-symmetric")]
    make_symmetric: bool,

    /// Output SVG file (default: heatmap.svg)
    #[arg(short = 'o', long = "output", default_value = "heatmap.svg")]
    output: String,
}

/// Generates an NxN matrix filled with random f64 values in the range [0.0, 1.0).
fn generate_random_matrix(n: usize) -> Vec<Vec<f64>> {
    let mut rng = rand::thread_rng();

    (0..n).map(|_| {
            (0..n).map(|_| rng.random::<f64>()).collect()
        }).collect()
}

/// Reads a matrix from a file using given column indices (zero-based).
/// `col_i`, `col_j`, `col_val` specify which columns correspond to row, column, and value.
fn read_matrix_three_columns<P: AsRef<Path>>(filename: P,
            col_i: usize, col_j: usize, col_val: usize, make_symmetric: bool) -> Result<Vec<Vec<f64>>, String> {
    let file = File::open(&filename).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut entries = Vec::new();
    let mut max_index = 0;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|e| e.to_string())?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue; // skip empty lines and comments
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        let max_col = col_i.max(col_j).max(col_val);
        if parts.len() <= max_col {
            return Err(format!(
                "Line {} does not contain enough columns (need at least {}): '{}'",
                line_num + 1, max_col + 1, line));
        }

        let i: usize = parts[col_i]
            .parse()
            .map_err(|_| format!("Invalid row index at line {}", line_num + 1))?;
        let j: usize = parts[col_j]
            .parse()
            .map_err(|_| format!("Invalid column index at line {}", line_num + 1))?;
        let value: f64 = parts[col_val]
            .parse()
            .map_err(|_| format!("Invalid value at line {}", line_num + 1))?;

        max_index = max_index.max(i).max(j);
        entries.push((i, j, value));
    }

    let n = max_index + 1;
    let mut matrix = vec![vec![0.0; n]; n];

    for (i, j, value) in entries {
        matrix[i][j] = value;
        if make_symmetric && i != j { matrix[j][i] = value; }
    }

    Ok(matrix)
}

/// Reads a matrix with row/column identifiers (strings) like "alice bob 3.3".
/// Uses provided column indices (zero-based): key_i, key_j, value.
pub fn read_matrix_annotated<P: AsRef<Path>>(filename: P,
                 col_i: usize, col_j: usize, col_val: usize, make_symmetric: bool) -> Result<Vec<Vec<f64>>, String> {
    let file = File::open(&filename).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut index_map: HashMap<String, usize> = HashMap::new();
    let mut entries = Vec::new();
    let mut current_index = 0;

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|e| e.to_string())?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();
        let max_col = col_i.max(col_j).max(col_val);

        if parts.len() <= max_col {
            return Err(format!(
                "Line {} does not contain enough columns (need at least {}): '{}'",
                line_num + 1,
                max_col + 1,
                line
            ));
        }

        let key_i = parts[col_i].to_string();
        let key_j = parts[col_j].to_string();
        let value: f64 = parts[col_val]
            .parse()
            .map_err(|_| format!("Invalid value at line {}", line_num + 1))?;

        let i = *index_map.entry(key_i).or_insert_with(|| {
            let idx = current_index;
            current_index += 1;
            idx
        });

        let j = *index_map.entry(key_j).or_insert_with(|| {
            let idx = current_index;
            current_index += 1;
            idx
        });

        entries.push((i, j, value));
    }

    let n = current_index;
    let mut matrix = vec![vec![0.0; n]; n];
    for (i, j, val) in entries {
        matrix[i][j] = val;
        if make_symmetric && i != j { matrix[j][i] = val; }
    }

    Ok(matrix)
}

/// Reads a square matrix from a file with a single column of values (row-wise order).
/// Skips empty lines and lines starting with `#`.
pub fn read_matrix_single_column<P: AsRef<Path>>(filename: P) -> Result<Vec<Vec<f64>>, String> {
    let file = File::open(&filename).map_err(|e| e.to_string())?;
    let reader = io::BufReader::new(file);

    let mut values = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line = line_result.map_err(|e| e.to_string())?;
        let line = line.trim();

        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let value: f64 = line.parse()
            .map_err(|_| format!("Invalid number at line {}: '{}'", line_num + 1, line))?;
        values.push(value);
    }

    let total = values.len();
    let n = (total as f64).sqrt() as usize;

    if n * n != total {
        return Err(format!(
            "Number of values ({}) is not a perfect square, cannot form square matrix", total
        ));
    }

    let mut matrix = Vec::with_capacity(n);
    for row in 0..n {
        let start = row * n;
        let end = start + n;
        matrix.push(values[start..end].to_vec());
    }

    Ok(matrix)
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let margin = 20.0;
    let box_size = 5.0;
    let data: Vec<Vec<f64>>;

    match &args.input {
        Some(filename) => {
            if args.single_column {
                data = read_matrix_single_column(filename)?;
                eprintln!("Loaded matrix from '{}' as single-column row-wise square matrix", filename);
            } else if args.annotated {
                let cols = args.columns.clone().unwrap_or_else(|| vec![1, 2, 3]);
                if cols.len() != 3 || cols.iter().any(|&c| c == 0) {
                    return Err("Column indices must be 1-based and non-zero".to_string());
                }
                let (i, j, val) = (cols[0] - 1, cols[1] - 1, cols[2] - 1);
                data = read_matrix_annotated(filename, i, j, val, args.make_symmetric)?;
                eprintln!(
                    "Loaded annotated matrix from '{}' using columns {} {} {}",
                    filename, cols[0], cols[1], cols[2]
                );
            } else {
                let cols = args.columns.clone().unwrap_or_else(|| vec![1, 2, 3]);
                if cols.len() != 3 || cols.iter().any(|&c| c == 0) {
                    return Err("Column indices must be 1-based and non-zero".to_string());
                }
                let (i, j, val) = (cols[0] - 1, cols[1] - 1, cols[2] - 1);
                data = read_matrix_three_columns(filename, i, j, val, args.make_symmetric)?;
                eprintln!(
                    "Loaded matrix from '{}' using columns {} {} {}",
                    filename, cols[0], cols[1], cols[2]
                );
            }
        }
        None => {
            let n = 30;
            data = generate_random_matrix(n);
            eprintln!("Generated random {}x{} matrix", n, n);
        }
    }

    let n = data.len();
    let draw_width = box_size * n as f32 + 2.0 * margin;
    let cmap = ColorMap::from_range(&RED_BLUE, 0.0, 1.0)?;
    let drawing = SvgDrawing::new(draw_width, draw_width);

    let map = Heatmap::from_data(drawing, "heatmap", box_size, box_size, data, &cmap);
    fs::write(&args.output, &map.to_svg()).map_err(|e| e.to_string())?;

    eprintln!("Saved {}", args.output);
    Ok(())
}
