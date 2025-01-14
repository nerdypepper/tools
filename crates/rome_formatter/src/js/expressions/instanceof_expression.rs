use crate::formatter_traits::FormatTokenAndNode;

use crate::{
    format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsInstanceofExpression;
use rome_js_syntax::JsInstanceofExpressionFields;

impl ToFormatElement for JsInstanceofExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsInstanceofExpressionFields {
            left,
            instanceof_token,
            right,
        } = self.as_fields();

        Ok(format_elements![
            left.format(formatter)?,
            space_token(),
            instanceof_token.format(formatter)?,
            space_token(),
            right.format(formatter)?,
        ])
    }
}
