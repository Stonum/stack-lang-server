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

/// Which real SQL dialect to accept.
///
/// `Standard` accepts a common core shared by real Postgres and T-SQL
/// (MSSQL), without either's own extras. `Postgres`/`Mssql` each
/// additionally accept that engine's own syntax on top of the common core
/// (`::` casts, `RETURNING`, `ON CONFLICT`, arrays, `ILIKE`, `LATERAL`,
/// `DISTINCT ON` for Postgres; `TOP`, `CROSS`/`OUTER APPLY`, T-SQL type
/// names for Mssql). Independent of [SqlExtensions] -- see its own doc
/// comment for why.
#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub enum SqlDialect {
    #[default]
    Standard,

    Postgres,

    Mssql,
}

impl SqlDialect {
    pub const fn is_standard(&self) -> bool {
        matches!(self, SqlDialect::Standard)
    }
    pub const fn is_postgres(&self) -> bool {
        matches!(self, SqlDialect::Postgres)
    }
    pub const fn is_mssql(&self) -> bool {
        matches!(self, SqlDialect::Mssql)
    }
}

/// Optional syntax extensions layered on top of whichever [SqlDialect] is
/// selected -- independent of it, unlike `Postgres`/`Mssql` themselves.
///
/// Currently just `mlang`: the conventions SQL embedded in mlang source
/// (`query("...")` calls) uses -- `~name~`/`~$name~` table/function names,
/// `~[]~` array-type-suffix escaping, `#name` temp table names, and
/// `[bracket]` identifiers. These are mlang's own runtime conventions, not
/// part of any real SQL dialect -- and (real-world confirmed) show up in
/// queries that are otherwise ordinary, valid Postgres, so they're modeled
/// as an extension orthogonal to the dialect rather than tied to one
/// (unlike, say, `[bracket]` identifiers being folded into `Mssql` --
/// legacy mlang queries use them under a Postgres-flavored dialect too).
#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub struct SqlExtensions {
    mlang: bool,
}

impl SqlExtensions {
    pub const fn with_mlang(mut self, mlang: bool) -> Self {
        self.mlang = mlang;
        self
    }

    pub const fn mlang(&self) -> bool {
        self.mlang
    }
}

#[derive(Debug, Clone, Default, Copy, Eq, PartialEq, Hash)]
pub struct SqlFileSource {
    module_kind: SqlModuleKind,
    dialect: SqlDialect,
    extensions: SqlExtensions,
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

    pub const fn with_extensions(mut self, extensions: SqlExtensions) -> Self {
        self.extensions = extensions;
        self
    }

    pub fn set_extensions(&mut self, extensions: SqlExtensions) {
        self.extensions = extensions;
    }

    pub const fn extensions(&self) -> SqlExtensions {
        self.extensions
    }

    pub const fn with_mlang_extension(mut self, mlang: bool) -> Self {
        self.extensions = self.extensions.with_mlang(mlang);
        self
    }

    pub const fn has_mlang_extension(&self) -> bool {
        self.extensions.mlang()
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
