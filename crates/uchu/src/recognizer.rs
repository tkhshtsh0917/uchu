use pyo3::prelude::*;

use super::token::Token;

/// Named entity recognizer for Japanese text.
#[pyclass]
pub(crate) struct JapaneseNERProcessor;

#[pymethods]
impl JapaneseNERProcessor {
    #[new]
    fn new() -> PyResult<Self> {
        Ok(Self)
    }

    fn recognize(&self, text: &str) -> PyResult<Vec<Token>> {
        let result = text.trim().split_whitespace().map(Token::new).collect();

        Ok(result)
    }
}
