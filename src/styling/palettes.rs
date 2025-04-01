

/// Tableau 10 palette (10 discrete categorical colors)
///
/// ...also known as Plotly 10 / D3 Category10 / Matplotlib tab10 palette (10 categorical colors)
/// <svg>
#[doc = include_str!("../../tests/expected_drawings/styling/tableau10.svg")]
/// </svg>
pub const TABLEAU10: [&str; 10] = [
    "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd",
    "#8c564b", "#e377c2", "#7f7f7f", "#bcbd22", "#17becf"
];

pub const CATEGORICAL_ACCENT: [&str; 10] = [
    "#4e79a7", "#f28e2c", "#e15759", "#76b7b2", "#59a14f",
    "#edc949", "#af7aa1", "#ff9da7", "#9c755f", "#bab0ab"
];

/// Tableau 20 palette (20 discrete categorical colors)
pub const TABLEAU20: [&str; 20] = [
    "#4e79a7", "#f28e2c", "#e15759", "#76b7b2", "#59a14f",
    "#edc949", "#af7aa1", "#ff9da7", "#9c755f", "#bab0ab",
    "#499894", "#8570b1", "#e7ba52", "#e78ac3", "#a87963",
    "#b2912f", "#6d8764", "#bdb2a1", "#8c613c", "#848482"];

/// Viridis palette
pub const VIRIDIS: [&str; 20] =  [
    "#440154", "#481567", "#482677", "#453781", "#404788",
    "#39568C", "#33638D", "#2D708E", "#287D8E", "#238A8D",
    "#1F968B", "#20A387", "#29AF7F", "#3CBB75", "#55C667",
    "#73D055", "#95D840", "#B8DE29", "#DCE319", "#FDE725"];

/// Pastel palette
pub const PASTEL: [&str; 9] =  [
    "#fbb4ae", "#b3cde3", "#ccebc5", "#decbe4", "#fed9a6", "#ffffcc", "#e5d8bd", "#fddaec", "#f2f2f2"];

/// Accent palette
pub const ACCENT: [&str; 8] =  [
    "#7fc97f", "#beaed4", "#fdc086", "#ffff99", "#386cb0", "#f0027f", "#bf5b17", "#666666"];

/// Palette of paired colors
pub const PAIRED: [&str; 12] =  [
    "#a6cee3", "#1f78b4", "#b2df8a", "#33a02c", "#fb9a99",
    "#e31a1c", "#fdbf6f", "#ff7f00", "#cab2d6", "#6a3d9a",
    "#ffff99", "#b15928"];

/// Okabe-Ito palette (8 colors, colorblind-safe)
pub const OKABE_ITO: [&str; 8] = [
    "#E69F00", "#56B4E9", "#009E73", "#F0E442",
    "#0072B2", "#D55E00", "#CC79A7", "#000000"
];

/// ggplot2 default discrete palette (8 colors)
pub const GGPLOT2_DEFAULT: [&str; 8] = [
    "#F8766D", "#7CAE00", "#00BFC4", "#C77CFF",
    "#E58700", "#00AFBB", "#FF61C3", "#A3A500"
];

/// IBM Design Language palette (8 categorical colors)
pub const IBM_COLORS: [&str; 8] = [
    "#648FFF", "#785EF0", "#DC267F", "#FE6100",
    "#FFB000", "#009E73", "#00BFC4", "#A3A500"
];

/// ColorBrewer Set1 palette (9 vibrant, colorblind-safe colors)
pub const COLORBREWER_SET1: [&str; 9] = [
    "#E41A1C", "#377EB8", "#4DAF4A", "#984EA3",
    "#FF7F00", "#FFFF33", "#A65628", "#F781BF", "#999999"
];

/// Generic dark categorical palette (8 colors)
pub const DARK: [&str; 8] = [
    "#1b1b1b", "#4e4e4e", "#6a3d9a", "#ff7f00",
    "#b15928", "#01665e", "#542788", "#e7298a"
];

/// ColorBrewer Dark2 palette (8 vivid, colorblind-friendly colors)
pub const DARK2: [&str; 8] = [
    "#1B9E77", "#D95F02", "#7570B3", "#E7298A",
    "#66A61E", "#E6AB02", "#A6761D", "#666666"
];

pub const RED_BLUE: [&str; 64] = [
    "#67001f", "#730421", "#7e0823", "#8a0c25", "#941127", "#9f172a", "#a81d2d",
    "#b12531", "#b82e35", "#bf373a", "#c6413f", "#cc4c45", "#d1574b", "#d66252",
    "#db6d59", "#e07861", "#e58369", "#e98d71", "#ed977a", "#f0a183", "#f3ab8d",
    "#f5b497", "#f7bda1", "#f9c5ab", "#faccb5", "#fad3bf", "#fbdac8", "#fae0d1",
    "#fae5d8", "#f9e9e0", "#f7ece6", "#f4eeeb", "#f1efee", "#edf0f1", "#e9eff2",
    "#e3edf2", "#ddeaf2", "#d6e7f0", "#cfe4ef", "#c7e0ed", "#bedbea", "#b5d7e8",
    "#abd1e5", "#a1cce2", "#96c6df", "#8bc0db", "#80b9d7", "#74b2d4", "#69aad0",
    "#5ea3cc", "#549bc8", "#4b94c4", "#428cc0", "#3b85bc", "#347eb7", "#2e76b2",
    "#296fad", "#2467a6", "#1f609e", "#1a5895", "#164f8b", "#114781", "#0d3f75",
    "#08366a"];