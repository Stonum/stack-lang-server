//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(bad_style, missing_docs, unreachable_pub)]
#[doc = r" The kind of syntax node, e.g. `IDENT`, `FUNCTION_KW`, or `FOR_STMT`."]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(u16)]
pub enum XmlSyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc = r" Marks the end of the file. May have trivia attached"]
    EOF,
    #[doc = r" Any Unicode BOM character that may be present at the start of"]
    #[doc = r" a file."]
    UNICODE_BOM,
    L_ANGLE,
    R_ANGLE,
    SLASH,
    EQ,
    HASH,
    XML_DECL_START,
    L_ANGLE_QUESTION,
    QUESTION_R_ANGLE,
    COMMENT_START,
    COMMENT_END,
    CDATA_START,
    CDATA_END,
    XML_STRING_LITERAL,
    XML_LITERAL,
    ERROR_TOKEN,
    NEWLINE,
    WHITESPACE,
    IDENT,
    XML_ROOT,
    XML_PROLOG,
    XML_ELEMENT_LIST,
    XML_ELEMENT,
    XML_SELF_CLOSING_ELEMENT,
    XML_OPENING_ELEMENT,
    XML_CLOSING_ELEMENT,
    XML_PROCESSING_INSTRUCTION,
    XML_COMMENT,
    XML_CDATA_SECTION,
    XML_ATTRIBUTE_LIST,
    XML_ATTRIBUTE,
    XML_ATTRIBUTE_INITIALIZER_CLAUSE,
    XML_NAME,
    XML_STRING,
    XML_TEXT,
    XML_BOGUS,
    XML_BOGUS_ELEMENT,
    XML_BOGUS_ATTRIBUTE,
    #[doc(hidden)]
    __LAST,
}
use self::XmlSyntaxKind::*;
impl XmlSyntaxKind {
    pub const fn is_punct(self) -> bool {
        matches!(
            self,
            L_ANGLE
                | R_ANGLE
                | SLASH
                | EQ
                | HASH
                | XML_DECL_START
                | L_ANGLE_QUESTION
                | QUESTION_R_ANGLE
                | COMMENT_START
                | COMMENT_END
                | CDATA_START
                | CDATA_END
        )
    }
    pub const fn is_literal(self) -> bool {
        matches!(self, XML_STRING_LITERAL | XML_LITERAL)
    }
    pub const fn is_list(self) -> bool {
        matches!(self, XML_ELEMENT_LIST | XML_ATTRIBUTE_LIST)
    }
    pub fn from_keyword(_ident: &str) -> Option<Self> {
        None
    }
    pub const fn to_string(&self) -> Option<&'static str> {
        let tok = match self {
            L_ANGLE => "<",
            R_ANGLE => ">",
            SLASH => "/",
            EQ => "=",
            HASH => "#",
            XML_DECL_START => "<?xml",
            L_ANGLE_QUESTION => "<?",
            QUESTION_R_ANGLE => "?>",
            COMMENT_START => "<!--",
            COMMENT_END => "-->",
            CDATA_START => "<![CDATA[",
            CDATA_END => "]]>",
            EOF => "EOF",
            XML_STRING_LITERAL => "string literal",
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = r" Utility macro for creating a SyntaxKind through simple macro syntax"]
#[macro_export]
macro_rules ! T { [<] => { $ crate :: XmlSyntaxKind :: L_ANGLE } ; [>] => { $ crate :: XmlSyntaxKind :: R_ANGLE } ; [/] => { $ crate :: XmlSyntaxKind :: SLASH } ; [=] => { $ crate :: XmlSyntaxKind :: EQ } ; [#] => { $ crate :: XmlSyntaxKind :: HASH } ; ["<?xml"] => { $ crate :: XmlSyntaxKind :: XML_DECL_START } ; [<?] => { $ crate :: XmlSyntaxKind :: L_ANGLE_QUESTION } ; [?>] => { $ crate :: XmlSyntaxKind :: QUESTION_R_ANGLE } ; [<!--] => { $ crate :: XmlSyntaxKind :: COMMENT_START } ; [-->] => { $ crate :: XmlSyntaxKind :: COMMENT_END } ; ["<![CDATA["] => { $ crate :: XmlSyntaxKind :: CDATA_START } ; ["]]>"] => { $ crate :: XmlSyntaxKind :: CDATA_END } ; [ident] => { $ crate :: XmlSyntaxKind :: IDENT } ; [EOF] => { $ crate :: XmlSyntaxKind :: EOF } ; [UNICODE_BOM] => { $ crate :: XmlSyntaxKind :: UNICODE_BOM } ; [#] => { $ crate :: XmlSyntaxKind :: HASH } ; }
