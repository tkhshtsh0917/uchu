use pyo3::prelude::*;

/// Named entity recognizer for Japanese text.
#[pyclass(eq, ord)]
#[derive(PartialEq, PartialOrd)]
pub(crate) struct Token {
    #[pyo3(get)]
    inner: String,
}

#[pymethods]
impl Token {
    #[new]
    pub(crate) fn new(inner: &str) -> Self {
        Self {
            inner: inner.to_string(),
        }
    }

    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("Token(value = {})", &self.inner))
    }

    fn __str__(&self) -> PyResult<&str> {
        Ok(&self.inner)
    }
}
