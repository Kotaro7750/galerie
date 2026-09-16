#[derive(Debug, Clone, PartialEq, Eq)]
/// Represents a cookie used for content access
pub(crate) struct ContentAccessCookie {
    name: String,
    value: String,
    domain: Option<String>,
    path: String,
}

impl ContentAccessCookie {
    pub(crate) fn new(name: String, value: String, domain: Option<String>, path: String) -> Self {
        Self {
            name,
            value,
            domain,
            path,
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn domain(&self) -> Option<&str> {
        self.domain.as_deref()
    }

    pub(crate) fn path(&self) -> &str {
        &self.path
    }
}
