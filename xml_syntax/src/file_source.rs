pub use biome_rowan::FileSourceError;
use std::{ffi::OsStr, path::Path};

/// Which flavour of Stack XML a document is.
///
/// `Resource` (`.rx`) and `Dictionary` (`.xdic`) are both plain XML; the
/// distinction only drives the domain-specific schema/semantic layer built
/// on top of the shared parser. `Plain` is a generic XML document with no
/// known schema.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub enum XmlVariant {
    #[default]
    Plain,
    Resource,
    Dictionary,
}

impl XmlVariant {
    pub const fn is_resource(&self) -> bool {
        matches!(self, XmlVariant::Resource)
    }
    pub const fn is_dictionary(&self) -> bool {
        matches!(self, XmlVariant::Dictionary)
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash)]
pub struct XmlFileSource {
    variant: XmlVariant,
}

impl XmlFileSource {
    pub fn plain() -> Self {
        Self::default()
    }

    pub fn resource() -> Self {
        Self {
            variant: XmlVariant::Resource,
        }
    }

    pub fn dictionary() -> Self {
        Self {
            variant: XmlVariant::Dictionary,
        }
    }

    pub const fn variant(&self) -> XmlVariant {
        self.variant
    }

    pub const fn with_variant(mut self, variant: XmlVariant) -> Self {
        self.variant = variant;
        self
    }

    /// Try to return the XML file source corresponding to this file extension.
    ///
    /// Resource files are `.rx` but also carry an arbitrary suffix in
    /// practice (`.rx_api`, `.rx_dialog`, `.rxz`, ...), so any extension
    /// starting with `rx` is treated as a resource. Dictionaries are always
    /// `.xdic`.
    pub fn try_from_extension(extension: &OsStr) -> Result<Self, FileSourceError> {
        // We assume the file extension is normalized to lowercase.
        let extension = extension
            .to_str()
            .ok_or(FileSourceError::UnknownExtension)?;

        match extension {
            "xml" => Ok(Self::plain()),
            "xdic" => Ok(Self::dictionary()),
            _ if extension.starts_with("rx") => Ok(Self::resource()),
            _ => Err(FileSourceError::UnknownExtension),
        }
    }
}

impl TryFrom<&Path> for XmlFileSource {
    type Error = FileSourceError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let extension = path
            .extension()
            // We assume the file extensions are case-insensitive.
            .map(|ext| ext.to_ascii_lowercase())
            .ok_or(FileSourceError::MissingFileExtension)?;

        Self::try_from_extension(&extension)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rx_extension_resolves_to_resource() {
        let source = XmlFileSource::try_from(Path::new("admin.rx")).unwrap();
        assert!(source.variant().is_resource());
    }

    #[test]
    fn xdic_extension_resolves_to_dictionary() {
        let source = XmlFileSource::try_from(Path::new("main.xdic")).unwrap();
        assert!(source.variant().is_dictionary());
    }

    #[test]
    fn extension_is_case_insensitive() {
        let source = XmlFileSource::try_from(Path::new("admin.RX")).unwrap();
        assert!(source.variant().is_resource());
    }

    #[test]
    fn suffixed_rx_extension_resolves_to_resource() {
        for name in ["admin.rx_api", "gis.rx_zi_gis_dlg", "menu.rxz", "z.rxi"] {
            let source = XmlFileSource::try_from(Path::new(name))
                .unwrap_or_else(|_| panic!("{name} should be a resource"));
            assert!(source.variant().is_resource(), "{name}");
        }
    }

    #[test]
    fn unrecognized_extension_is_an_error() {
        assert!(XmlFileSource::try_from(Path::new("script.prg")).is_err());
    }

    #[test]
    fn missing_extension_is_an_error() {
        assert!(XmlFileSource::try_from(Path::new("admin")).is_err());
    }
}
