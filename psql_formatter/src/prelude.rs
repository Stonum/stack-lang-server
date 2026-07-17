//! This module provides important and useful traits to help to format tokens and nodes
//! when implementing a syntax formatter.

// Scaffolding stage: the stub rule files generated for every node kind just
// call `format_verbatim_node`, so several of these re-exports have no users
// yet. They'll pick up users as real formatting rules replace the stubs.
#![allow(unused_imports)]

pub(crate) use super::AsFormat;
pub(crate) use super::IntoFormat;
pub(crate) use super::{
    FormatNodeRule, FormattedIterExt, PsqlFormatContext, PsqlFormatter, comments::PsqlComments,
};
pub use biome_formatter::prelude::*;
pub use biome_formatter::separated::TrailingSeparator;
pub use biome_rowan::{AstNode as _, AstNodeList as _, AstSeparatedList as _};

pub(crate) use super::macros::{impl_format, impl_format_with_rule, impl_rule};
pub(crate) use super::separated::FormatAstSeparatedListExtension;
