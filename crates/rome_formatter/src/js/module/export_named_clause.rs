use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};

use crate::{
    format_elements, space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsExportNamedClause;
use rome_js_syntax::JsExportNamedClauseFields;

impl ToFormatElement for JsExportNamedClause {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExportNamedClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            semicolon_token,
        } = self.as_fields();

        let type_token = type_token
            .format_with_or_empty(formatter, |token| format_elements![token, space_token()])?;

        let specifiers = specifiers.format(formatter)?;

        let list = formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            specifiers,
            &r_curly_token?,
        )?;

        let semicolon = semicolon_token.format_or(formatter, || token(";"))?;

        Ok(format_elements![type_token, list, semicolon])
    }
}
