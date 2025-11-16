use pyo3::prelude::*;

pub(crate) mod py_heatmap;
pub use py_heatmap::PyHeatmap;

pub fn init_submodule(parent_module: &Bound<'_, PyModule>) -> PyResult<()> {
    let m = PyModule::new(parent_module.py(), "heatmap")?;
    m.add_class::<PyHeatmap>()?;
    parent_module.add_submodule(&m)
}
