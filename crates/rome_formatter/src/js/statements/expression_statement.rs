use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{format_elements, token, FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsExpressionStatementFields;

impl ToFormatElement for JsExpressionStatement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExpressionStatementFields {
            expression,
            semicolon_token,
        } = self.as_fields();

        Ok(format_elements![
            expression.format(formatter)?,
            semicolon_token.format_or(formatter, || token(";"))?
        ])
    }
}
