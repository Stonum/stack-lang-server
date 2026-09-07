//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::XmlSyntaxNode::kind(&node) {
                $crate::XmlSyntaxKind::XML_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::XmlAttribute::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_ATTRIBUTE_INITIALIZER_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::XmlAttributeInitializerClause::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_CDATA_SECTION => {
                    let $pattern = unsafe { $crate::XmlCdataSection::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_CLOSING_ELEMENT => {
                    let $pattern = unsafe { $crate::XmlClosingElement::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_COMMENT => {
                    let $pattern = unsafe { $crate::XmlComment::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_ELEMENT => {
                    let $pattern = unsafe { $crate::XmlElement::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_NAME => {
                    let $pattern = unsafe { $crate::XmlName::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_OPENING_ELEMENT => {
                    let $pattern = unsafe { $crate::XmlOpeningElement::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_PROCESSING_INSTRUCTION => {
                    let $pattern = unsafe { $crate::XmlProcessingInstruction::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_PROLOG => {
                    let $pattern = unsafe { $crate::XmlProlog::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_ROOT => {
                    let $pattern = unsafe { $crate::XmlRoot::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_SELF_CLOSING_ELEMENT => {
                    let $pattern = unsafe { $crate::XmlSelfClosingElement::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_STRING => {
                    let $pattern = unsafe { $crate::XmlString::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_TEXT => {
                    let $pattern = unsafe { $crate::XmlText::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_BOGUS => {
                    let $pattern = unsafe { $crate::XmlBogus::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_BOGUS_ATTRIBUTE => {
                    let $pattern = unsafe { $crate::XmlBogusAttribute::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_BOGUS_ELEMENT => {
                    let $pattern = unsafe { $crate::XmlBogusElement::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_ATTRIBUTE_LIST => {
                    let $pattern = unsafe { $crate::XmlAttributeList::new_unchecked(node) };
                    $body
                }
                $crate::XmlSyntaxKind::XML_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::XmlElementList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
