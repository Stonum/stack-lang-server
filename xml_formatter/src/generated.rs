//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, XmlFormatContext, XmlFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<xml_syntax::XmlAttribute> for crate::xml::auxiliary::attribute::FormatXmlAttribute {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlAttribute, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlAttribute>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlAttribute,
        crate::xml::auxiliary::attribute::FormatXmlAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::attribute::FormatXmlAttribute::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlAttribute {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlAttribute,
        crate::xml::auxiliary::attribute::FormatXmlAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::attribute::FormatXmlAttribute::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlAttributeInitializerClause>
    for crate::xml::auxiliary::attribute_initializer_clause::FormatXmlAttributeInitializerClause
{
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &xml_syntax::XmlAttributeInitializerClause,
        f: &mut XmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlAttributeInitializerClause>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlAttributeInitializerClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlAttributeInitializerClause,
        crate::xml::auxiliary::attribute_initializer_clause::FormatXmlAttributeInitializerClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: xml :: auxiliary :: attribute_initializer_clause :: FormatXmlAttributeInitializerClause :: default ())
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlAttributeInitializerClause {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlAttributeInitializerClause,
        crate::xml::auxiliary::attribute_initializer_clause::FormatXmlAttributeInitializerClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: xml :: auxiliary :: attribute_initializer_clause :: FormatXmlAttributeInitializerClause :: default ())
    }
}
impl FormatRule<xml_syntax::XmlCdataSection>
    for crate::xml::auxiliary::cdata_section::FormatXmlCdataSection
{
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlCdataSection, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlCdataSection>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlCdataSection {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlCdataSection,
        crate::xml::auxiliary::cdata_section::FormatXmlCdataSection,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::cdata_section::FormatXmlCdataSection::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlCdataSection {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlCdataSection,
        crate::xml::auxiliary::cdata_section::FormatXmlCdataSection,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::cdata_section::FormatXmlCdataSection::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlClosingElement>
    for crate::xml::auxiliary::closing_element::FormatXmlClosingElement
{
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlClosingElement, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlClosingElement>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlClosingElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlClosingElement,
        crate::xml::auxiliary::closing_element::FormatXmlClosingElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::closing_element::FormatXmlClosingElement::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlClosingElement {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlClosingElement,
        crate::xml::auxiliary::closing_element::FormatXmlClosingElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::closing_element::FormatXmlClosingElement::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlComment> for crate::xml::auxiliary::comment::FormatXmlComment {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlComment, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlComment>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlComment {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlComment,
        crate::xml::auxiliary::comment::FormatXmlComment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::comment::FormatXmlComment::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlComment {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlComment,
        crate::xml::auxiliary::comment::FormatXmlComment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::comment::FormatXmlComment::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlElement> for crate::xml::auxiliary::element::FormatXmlElement {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlElement, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlElement>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlElement,
        crate::xml::auxiliary::element::FormatXmlElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::element::FormatXmlElement::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlElement {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlElement,
        crate::xml::auxiliary::element::FormatXmlElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::element::FormatXmlElement::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlName> for crate::xml::auxiliary::name::FormatXmlName {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlName, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlName>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlName {
    type Format<'a> =
        FormatRefWithRule<'a, xml_syntax::XmlName, crate::xml::auxiliary::name::FormatXmlName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::xml::auxiliary::name::FormatXmlName::default())
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlName {
    type Format =
        FormatOwnedWithRule<xml_syntax::XmlName, crate::xml::auxiliary::name::FormatXmlName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::xml::auxiliary::name::FormatXmlName::default())
    }
}
impl FormatRule<xml_syntax::XmlOpeningElement>
    for crate::xml::auxiliary::opening_element::FormatXmlOpeningElement
{
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlOpeningElement, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlOpeningElement>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlOpeningElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlOpeningElement,
        crate::xml::auxiliary::opening_element::FormatXmlOpeningElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::opening_element::FormatXmlOpeningElement::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlOpeningElement {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlOpeningElement,
        crate::xml::auxiliary::opening_element::FormatXmlOpeningElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::opening_element::FormatXmlOpeningElement::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlProcessingInstruction>
    for crate::xml::auxiliary::processing_instruction::FormatXmlProcessingInstruction
{
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &xml_syntax::XmlProcessingInstruction,
        f: &mut XmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlProcessingInstruction>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlProcessingInstruction {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlProcessingInstruction,
        crate::xml::auxiliary::processing_instruction::FormatXmlProcessingInstruction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::processing_instruction::FormatXmlProcessingInstruction::default(
            ),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlProcessingInstruction {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlProcessingInstruction,
        crate::xml::auxiliary::processing_instruction::FormatXmlProcessingInstruction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::processing_instruction::FormatXmlProcessingInstruction::default(
            ),
        )
    }
}
impl FormatRule<xml_syntax::XmlProlog> for crate::xml::auxiliary::prolog::FormatXmlProlog {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlProlog, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlProlog>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlProlog {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlProlog,
        crate::xml::auxiliary::prolog::FormatXmlProlog,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::prolog::FormatXmlProlog::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlProlog {
    type Format =
        FormatOwnedWithRule<xml_syntax::XmlProlog, crate::xml::auxiliary::prolog::FormatXmlProlog>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::prolog::FormatXmlProlog::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlRoot> for crate::xml::auxiliary::root::FormatXmlRoot {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlRoot, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlRoot>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlRoot {
    type Format<'a> =
        FormatRefWithRule<'a, xml_syntax::XmlRoot, crate::xml::auxiliary::root::FormatXmlRoot>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::xml::auxiliary::root::FormatXmlRoot::default())
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlRoot {
    type Format =
        FormatOwnedWithRule<xml_syntax::XmlRoot, crate::xml::auxiliary::root::FormatXmlRoot>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::xml::auxiliary::root::FormatXmlRoot::default())
    }
}
impl FormatRule<xml_syntax::XmlSelfClosingElement>
    for crate::xml::auxiliary::self_closing_element::FormatXmlSelfClosingElement
{
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &xml_syntax::XmlSelfClosingElement,
        f: &mut XmlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlSelfClosingElement>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlSelfClosingElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlSelfClosingElement,
        crate::xml::auxiliary::self_closing_element::FormatXmlSelfClosingElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::self_closing_element::FormatXmlSelfClosingElement::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlSelfClosingElement {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlSelfClosingElement,
        crate::xml::auxiliary::self_closing_element::FormatXmlSelfClosingElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::self_closing_element::FormatXmlSelfClosingElement::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlString> for crate::xml::auxiliary::string::FormatXmlString {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlString, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlString>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlString {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlString,
        crate::xml::auxiliary::string::FormatXmlString,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::auxiliary::string::FormatXmlString::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlString {
    type Format =
        FormatOwnedWithRule<xml_syntax::XmlString, crate::xml::auxiliary::string::FormatXmlString>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::auxiliary::string::FormatXmlString::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlText> for crate::xml::auxiliary::text::FormatXmlText {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlText, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatNodeRule::<xml_syntax::XmlText>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlText {
    type Format<'a> =
        FormatRefWithRule<'a, xml_syntax::XmlText, crate::xml::auxiliary::text::FormatXmlText>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::xml::auxiliary::text::FormatXmlText::default())
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlText {
    type Format =
        FormatOwnedWithRule<xml_syntax::XmlText, crate::xml::auxiliary::text::FormatXmlText>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::xml::auxiliary::text::FormatXmlText::default())
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlAttributeList {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlAttributeList,
        crate::xml::lists::attribute_list::FormatXmlAttributeList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::lists::attribute_list::FormatXmlAttributeList::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlAttributeList {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlAttributeList,
        crate::xml::lists::attribute_list::FormatXmlAttributeList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::lists::attribute_list::FormatXmlAttributeList::default(),
        )
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlElementList,
        crate::xml::lists::element_list::FormatXmlElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::lists::element_list::FormatXmlElementList::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlElementList {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlElementList,
        crate::xml::lists::element_list::FormatXmlElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::lists::element_list::FormatXmlElementList::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlBogus> for crate::xml::bogus::bogus::FormatXmlBogus {
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlBogus, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<xml_syntax::XmlBogus>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlBogus {
    type Format<'a> =
        FormatRefWithRule<'a, xml_syntax::XmlBogus, crate::xml::bogus::bogus::FormatXmlBogus>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::xml::bogus::bogus::FormatXmlBogus::default())
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlBogus {
    type Format =
        FormatOwnedWithRule<xml_syntax::XmlBogus, crate::xml::bogus::bogus::FormatXmlBogus>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::xml::bogus::bogus::FormatXmlBogus::default())
    }
}
impl FormatRule<xml_syntax::XmlBogusAttribute>
    for crate::xml::bogus::bogus_attribute::FormatXmlBogusAttribute
{
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlBogusAttribute, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<xml_syntax::XmlBogusAttribute>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlBogusAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlBogusAttribute,
        crate::xml::bogus::bogus_attribute::FormatXmlBogusAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::bogus::bogus_attribute::FormatXmlBogusAttribute::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlBogusAttribute {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlBogusAttribute,
        crate::xml::bogus::bogus_attribute::FormatXmlBogusAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::bogus::bogus_attribute::FormatXmlBogusAttribute::default(),
        )
    }
}
impl FormatRule<xml_syntax::XmlBogusElement>
    for crate::xml::bogus::bogus_element::FormatXmlBogusElement
{
    type Context = XmlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &xml_syntax::XmlBogusElement, f: &mut XmlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<xml_syntax::XmlBogusElement>::fmt(self, node, f)
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::XmlBogusElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::XmlBogusElement,
        crate::xml::bogus::bogus_element::FormatXmlBogusElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::bogus::bogus_element::FormatXmlBogusElement::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::XmlBogusElement {
    type Format = FormatOwnedWithRule<
        xml_syntax::XmlBogusElement,
        crate::xml::bogus::bogus_element::FormatXmlBogusElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::bogus::bogus_element::FormatXmlBogusElement::default(),
        )
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::AnyXmlAttribute {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::AnyXmlAttribute,
        crate::xml::any::attribute::FormatAnyXmlAttribute,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::any::attribute::FormatAnyXmlAttribute::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::AnyXmlAttribute {
    type Format = FormatOwnedWithRule<
        xml_syntax::AnyXmlAttribute,
        crate::xml::any::attribute::FormatAnyXmlAttribute,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::any::attribute::FormatAnyXmlAttribute::default(),
        )
    }
}
impl AsFormat<XmlFormatContext> for xml_syntax::AnyXmlElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        xml_syntax::AnyXmlElement,
        crate::xml::any::element::FormatAnyXmlElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::xml::any::element::FormatAnyXmlElement::default(),
        )
    }
}
impl IntoFormat<XmlFormatContext> for xml_syntax::AnyXmlElement {
    type Format = FormatOwnedWithRule<
        xml_syntax::AnyXmlElement,
        crate::xml::any::element::FormatAnyXmlElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::xml::any::element::FormatAnyXmlElement::default(),
        )
    }
}
