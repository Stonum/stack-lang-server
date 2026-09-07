//! Traits used when implementing the XML formatter.

// Scaffolding stage: the stub rule files generated for every node kind just
// call `format_verbatim_node`, so several of these re-exports have no users
// yet. They pick up users as real rules replace the stubs.
#![allow(unused_imports)]

pub(crate) use super::AsFormat;
pub(crate) use super::IntoFormat;
pub(crate) use super::{
    FormatNodeRule, FormattedIterExt, XmlFormatContext, XmlFormatter, comments::XmlComments,
};
pub use biome_formatter::prelude::*;
pub use biome_rowan::{AstNode as _, AstNodeList as _};

pub(crate) use super::macros::{impl_format, impl_format_with_rule, impl_rule};
