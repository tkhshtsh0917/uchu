mod recognizer;
mod token;

use pyo3::prelude::*;

use recognizer::JapaneseNERProcessor;
use token::Token;

/// A Python module implemented in Rust.
#[pymodule]
fn uchu(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Token>()?;
    m.add_class::<JapaneseNERProcessor>()?;

    Ok(())
}
