use crate::{SqlRoot, SqlSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct SqlLanguage;

impl Language for SqlLanguage {
    type Kind = SqlSyntaxKind;
    type Root = SqlRoot;
}

pub type SqlSyntaxNode = biome_rowan::SyntaxNode<SqlLanguage>;
pub type SqlSyntaxToken = biome_rowan::SyntaxToken<SqlLanguage>;
pub type SqlSyntaxElement = biome_rowan::SyntaxElement<SqlLanguage>;
pub type SqlSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<SqlLanguage>;
pub type SqlSyntaxElementChildren = biome_rowan::SyntaxElementChildren<SqlLanguage>;
pub type SqlSyntaxList = biome_rowan::SyntaxList<SqlLanguage>;
pub type SqlSyntaxTrivia = biome_rowan::syntax::SyntaxTrivia<SqlLanguage>;
