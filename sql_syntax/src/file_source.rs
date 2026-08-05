pub use biome_rowan::FileSourceError;
use std::{ffi::OsStr, path::Path};

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub enum SqlModuleKind {
    #[default]
    Query,

    Script,
}

impl SqlModuleKind {
    pub const fn is_script(&self) -> bool {
        matches!(self, SqlModuleKind::Script)
    }
    pub const fn is_query(&self) -> bool {
        matches!(self, SqlModuleKind::Query)
    }
}

/// Which SQL dialect to accept.
///
/// `Standard` accepts plain Postgres syntax. `Mlang` additionally accepts
/// syntax specific to SQL embedded in mlang source (e.g. `query("...")`
/// calls): `~name~`/`~$name~` table and function names, `#name` temp table
/// names, and `~[]~` as an escaped array-type suffix. These aren't real
/// Postgres syntax -- they're mlang's own conventions, presumably expanded
/// by mlang's runtime before the query text reaches an actual database.
#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub enum SqlDialect {
    #[default]
    Standard,

    Mlang,
}

impl SqlDialect {
    pub const fn is_mlang(&self) -> bool {
        matches!(self, SqlDialect::Mlang)
    }
}

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub struct SqlFileSource {
    module_kind: SqlModuleKind,
    dialect: SqlDialect,
}

impl SqlFileSource {
    pub fn query() -> Self {
        Self::default()
    }

    pub fn script() -> Self {
        Self::default().with_module_kind(SqlModuleKind::Script)
    }

    pub const fn with_module_kind(mut self, kind: SqlModuleKind) -> Self {
        self.module_kind = kind;
        self
    }

    pub fn set_module_kind(&mut self, kind: SqlModuleKind) {
        self.module_kind = kind;
    }

    pub const fn module_kind(&self) -> SqlModuleKind {
        self.module_kind
    }

    pub const fn is_query(&self) -> bool {
        self.module_kind.is_query()
    }

    pub const fn is_script(&self) -> bool {
        self.module_kind.is_script()
    }

    pub const fn with_dialect(mut self, dialect: SqlDialect) -> Self {
        self.dialect = dialect;
        self
    }

    pub fn set_dialect(&mut self, dialect: SqlDialect) {
        self.dialect = dialect;
    }

    pub const fn dialect(&self) -> SqlDialect {
        self.dialect
    }

    pub const fn is_mlang_dialect(&self) -> bool {
        self.dialect.is_mlang()
    }

    pub fn file_extension(&self) -> &str {
        match self.module_kind {
            SqlModuleKind::Query => "",
            SqlModuleKind::Script => "sql",
        }
    }

    /// Try to return the sql file source corresponding to this file extension
    pub fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase
        match extension.as_encoded_bytes() {
            b"sql" => Ok(Self::script()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }
}

impl TryFrom<&Path> for SqlFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let extension = path
            .extension()
            // We assume the file extensions are case-insensitive.
            // Thus, we normalize the extension to lowercase.
            .map(|ext| ext.to_ascii_lowercase())
            .ok_or(FileSourceError::MissingFileExtension)?;

        Self::try_from_extension(&extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sql_extension_resolves_to_script() {
        let source = SqlFileSource::try_from(Path::new("query.sql")).unwrap();
        assert!(source.is_script());
    }

    #[test]
    fn sql_extension_is_case_insensitive() {
        let source = SqlFileSource::try_from(Path::new("query.SQL")).unwrap();
        assert!(source.is_script());
    }

    #[test]
    fn unrecognized_extension_is_an_error() {
        // Must genuinely fail (not silently fall back to `query()`) so
        // callers can use `try_from` to decide "is this a sql file at
        // all", e.g. dispatching between mlang and sql document types.
        assert!(SqlFileSource::try_from(Path::new("script.prg")).is_err());
    }

    #[test]
    fn missing_extension_is_an_error() {
        assert!(SqlFileSource::try_from(Path::new("query")).is_err());
    }
}
