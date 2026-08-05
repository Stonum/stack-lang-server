use biome_rowan::TreeBuilder;
use sql_syntax::SqlLanguage;

mod generated;
pub mod make;
pub use generated::SqlSyntaxFactory;

pub type SqlSyntaxTreeBuilder = TreeBuilder<'static, SqlLanguage, SqlSyntaxFactory>;
