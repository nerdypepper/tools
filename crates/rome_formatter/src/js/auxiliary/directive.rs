use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsDirective;
use rome_js_syntax::JsDirectiveFields;

impl ToFormatElement for JsDirective {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDirectiveFields {
            value_token,
            semicolon_token,
        } = self.as_fields();

        Ok(format_elements![
            value_token.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?,
        ])
    }
}
