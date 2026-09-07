use biome_rowan::TreeBuilder;
use xml_syntax::XmlLanguage;

mod generated;
pub mod make;
pub use generated::XmlSyntaxFactory;

pub type XmlSyntaxTreeBuilder = TreeBuilder<'static, XmlLanguage, XmlSyntaxFactory>;
