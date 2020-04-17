use pyo3::prelude::{pymodule, PyModule, PyResult, Python};
mod hash;
mod list;
mod vector;

#[pymodule]
fn pyrpds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<list::PyList>()?;
    m.add_class::<vector::PyVector>()?;

    Ok(())
}