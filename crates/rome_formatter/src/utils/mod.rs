pub(crate) mod array;
mod binarish_expression;
mod call_expression;
mod format_conditional;
mod simple;

use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    empty_element, format_elements, hard_group_elements, hard_line_break, space_token, token,
    FormatElement, FormatResult, Formatter, Token,
};
pub use binarish_expression::format_binaryish_expression;
pub(crate) use call_expression::format_call_expression;
pub(crate) use format_conditional::{format_conditional, Conditional};
use rome_js_syntax::{AstNode, AstNodeList, SyntaxNode, SyntaxNodeExt, SyntaxToken};
use rome_js_syntax::{
    JsAnyExpression, JsAnyFunction, JsAnyRoot, JsAnyStatement, JsInitializerClause,
    JsTemplateElement, JsTemplateElementFields, Modifiers, TsTemplateElement,
    TsTemplateElementFields, TsType,
};

use crate::format_element::normalize_newlines;
pub(crate) use simple::*;

/// Utility function to format the separators of the nodes that belong to the unions
/// of [rome_js_syntax::TsAnyTypeMember].
///
/// We can have two kind of separators: `,`, `;` or ASI.
/// Because of how the grammar crafts the nodes, the parent will add the separator to the node.
/// So here, we create - on purpose - an empty node.
pub(crate) fn format_type_member_separator(
    separator_token: Option<SyntaxToken>,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    if let Some(separator) = separator_token {
        formatter.format_replaced(&separator, empty_element())
    } else {
        Ok(empty_element())
    }
}

/// Utility function to format the node [rome_js_syntax::JsInitializerClause]
pub(crate) fn format_initializer_clause(
    formatter: &Formatter,
    initializer: Option<JsInitializerClause>,
) -> FormatResult<FormatElement> {
    initializer.format_with_or_empty(formatter, |initializer| {
        format_elements![space_token(), initializer]
    })
}

pub(crate) fn format_interpreter(
    interpreter: Option<SyntaxToken>,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    interpreter.format_with_or(
        formatter,
        |interpreter| format_elements![interpreter, hard_line_break()],
        empty_element,
    )
}

/// Returns true if this node contains "printable" trivias: comments
/// or empty lines (2 consecutive newlines only separated by whitespace)
pub(crate) fn has_formatter_trivia(node: &SyntaxNode) -> bool {
    let mut line_count = 0;

    for token in node.tokens() {
        for trivia in token.leading_trivia().pieces() {
            if trivia.is_comments() {
                return true;
            } else if trivia.is_newline() {
                line_count += 1;
                if line_count >= 2 {
                    return true;
                }
            }
        }

        // This is where the token would be,
        // reset the consecutive newline counter
        line_count = 0;

        for trivia in token.trailing_trivia().pieces() {
            if trivia.is_comments() {
                return true;
            } else if trivia.is_newline() {
                line_count += 1;
                if line_count >= 2 {
                    return true;
                }
            }
        }
    }

    false
}

/// Format an element with a single line head and a body that might
/// be either a block or a single statement
///
/// This will place the head element inside a [hard_group_elements], but
/// the body will broken out of flat printing if its a single statement
pub(crate) fn format_head_body_statement(
    formatter: &Formatter,
    head: FormatElement,
    body: JsAnyStatement,
) -> FormatResult<FormatElement> {
    if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
        Ok(hard_group_elements(format_elements![
            head,
            space_token(),
            body.format(formatter)?,
        ]))
    } else if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
        // Force semicolon insertion if the body is empty
        Ok(format_elements![
            hard_group_elements(head),
            body.format(formatter)?,
            token(";"),
        ])
    } else {
        Ok(format_elements![
            hard_group_elements(head),
            space_token(),
            body.format(formatter)?,
        ])
    }
}

/// Single instance of a suppression comment, with the following syntax:
///
/// `// rome-ignore { <category> { (<value>) }? }+: <reason>`
///
/// The category broadly describes what feature is being suppressed (formatting,
/// linting, ...) with the value being and optional, category-specific name of
/// a specific element to disable (for instance a specific lint name). A single
/// suppression may specify one or more categories + values, for instance to
/// disable multiple lints at once
///
/// A suppression must specify a reason: this part has no semantic meaning but
/// is required to document why a particular feature is being disable for this
/// line (lint false-positive, specific formatting requirements, ...)
#[derive(Debug, PartialEq, Eq)]
struct Suppression<'a> {
    /// List of categories for this suppression
    ///
    /// Categories are pair of the category name +
    /// an optional category value
    categories: Vec<(&'a str, Option<&'a str>)>,
    /// Reason for this suppression comment to exist
    reason: &'a str,
}

const CATEGORY_FORMAT: &str = "format";

fn parse_suppression_comment(comment: &str) -> impl Iterator<Item = Suppression> {
    let (head, mut comment) = comment.split_at(2);
    let is_block_comment = match head {
        "//" => false,
        "/*" => {
            comment = comment
                .strip_suffix("*/")
                .expect("block comment with no closing token");
            true
        }
        token => panic!("comment with unknown opening token {token:?}"),
    };

    comment.lines().filter_map(move |line| {
        // Eat start of line whitespace
        let mut line = line.trim_start();

        // If we're in a block comment eat stars, then whitespace again
        if is_block_comment {
            line = line.trim_start_matches('*').trim_start()
        }

        // Check for the rome-ignore token or skip the line entirely
        line = line.strip_prefix("rome-ignore")?.trim_start();

        let mut categories = Vec::new();

        loop {
            // Find either a colon opening parenthesis or space
            let separator = line.find(|c: char| c == ':' || c == '(' || c.is_whitespace())?;

            let (category, rest) = line.split_at(separator);
            let category = category.trim_end();

            // Skip over and match the separator
            let (separator, rest) = rest.split_at(1);

            match separator {
                // Colon token: stop parsing categories
                ":" => {
                    if !category.is_empty() {
                        categories.push((category, None));
                    }

                    line = rest.trim_start();
                    break;
                }
                // Paren token: parse a category + value
                "(" => {
                    let paren = rest.find(')')?;

                    let (value, rest) = rest.split_at(paren);
                    let value = value.trim();

                    categories.push((category, Some(value)));

                    line = rest.strip_prefix(')').unwrap().trim_start();
                }
                // Whitespace: push a category without value
                _ => {
                    if !category.is_empty() {
                        categories.push((category, None));
                    }

                    line = rest.trim_start();
                }
            }
        }

        let reason = line.trim_end();
        Some(Suppression { categories, reason })
    })
}

pub(crate) fn has_formatter_suppressions(node: &SyntaxNode) -> bool {
    // Lists cannot have a suppression comment attached, it must
    // belong to either the entire parent node or one of the children
    let kind = node.kind();
    if JsAnyRoot::can_cast(kind) || kind.is_list() {
        return false;
    }

    let first_token = match node.first_token() {
        Some(token) => token,
        None => return false,
    };

    first_token
        .leading_trivia()
        .pieces()
        .filter_map(|trivia| trivia.as_comments())
        .any(|comment| {
            for suppression in parse_suppression_comment(comment.text()) {
                for (category, _) in suppression.categories {
                    if category == CATEGORY_FORMAT {
                        return true;
                    }
                }
            }

            false
        })
}

#[cfg(test)]
mod tests {
    use super::{parse_suppression_comment, Suppression};

    #[test]
    fn parse_simple_suppression() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore parse: explanation1").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", None)],
                reason: "explanation1"
            }],
        );

        assert_eq!(
            parse_suppression_comment("/** rome-ignore parse: explanation2 */").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", None)],
                reason: "explanation2"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * rome-ignore parse: explanation3
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", None)],
                reason: "explanation3"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * rome-ignore parse: explanation4
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", None)],
                reason: "explanation4"
            }],
        );
    }

    #[test]
    fn parse_multiple_suppression() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore parse(foo) parse(dog): explanation")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", Some("foo")), ("parse", Some("dog"))],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment("/** rome-ignore parse(bar) parse(cat): explanation */")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", Some("bar")), ("parse", Some("cat"))],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * rome-ignore parse(yes) parse(frog): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", Some("yes")), ("parse", Some("frog"))],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
                  * hello
                  * rome-ignore parse(wow) parse(fish): explanation
                  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", Some("wow")), ("parse", Some("fish"))],
                reason: "explanation"
            }],
        );
    }

    #[test]
    fn parse_multiple_suppression_categories() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("format", None), ("lint", None)],
                reason: "explanation"
            }],
        );
    }
}

/// This function consumes a list of modifiers and applies a predictable sorting.
pub(crate) fn sort_modifiers_by_precedence<List, Node>(list: &List) -> Vec<Node>
where
    Node: AstNode + Clone,
    List: AstNodeList<Node>,
    Modifiers: for<'a> From<&'a Node>,
{
    let mut nodes_and_modifiers = list.iter().collect::<Vec<Node>>();

    nodes_and_modifiers.sort_unstable_by_key(|node| Modifiers::from(node));

    nodes_and_modifiers
}

/// Utility to format
pub(crate) fn format_template_chunk(
    chunk: SyntaxToken,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    // Per https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-static-semantics-trv:
    // In template literals, the '\r' and '\r\n' line terminators are normalized to '\n'
    formatter.format_replaced(
        &chunk,
        FormatElement::from(Token::new_dynamic(
            normalize_newlines(chunk.text_trimmed(), ['\r']).into_owned(),
            chunk.text_trimmed_range(),
        )),
    )
}

/// Function to format template literals and template literal types
pub(crate) fn format_template_literal(
    literal: TemplateElement,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    literal.into_format_element(formatter)
}

pub(crate) enum TemplateElement {
    Js(JsTemplateElement),
    Ts(TsTemplateElement),
}

impl TemplateElement {
    pub fn into_format_element(self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let expression_is_plain = self.is_plain_expression()?;
        let has_comments = self.has_comments();
        let should_hard_group = expression_is_plain && !has_comments;

        let (dollar_curly_token, middle, r_curly_token) = match self {
            TemplateElement::Js(template_element) => {
                let JsTemplateElementFields {
                    dollar_curly_token,
                    expression,
                    r_curly_token,
                } = template_element.as_fields();

                let dollar_curly_token = dollar_curly_token?;
                let expression = expression.format(formatter)?;
                let r_curly_token = r_curly_token?;

                (dollar_curly_token, expression, r_curly_token)
            }
            TemplateElement::Ts(template_element) => {
                let TsTemplateElementFields {
                    ty,
                    r_curly_token,
                    dollar_curly_token,
                } = template_element.as_fields();

                let dollar_curly_token = dollar_curly_token?;
                let ty = ty.format(formatter)?;
                let r_curly_token = r_curly_token?;

                (dollar_curly_token, ty, r_curly_token)
            }
        };

        if should_hard_group {
            Ok(hard_group_elements(format_elements![
                dollar_curly_token.format(formatter)?,
                middle,
                r_curly_token.format(formatter)?
            ]))
        } else {
            formatter.format_delimited_soft_block_indent(
                &dollar_curly_token,
                middle,
                &r_curly_token,
            )
        }
    }

    /// We want to break the template element only when we have articulated expressions inside it.
    ///
    /// We a plain expression is when it's one of the following:
    /// - `loreum ${this.something} ipsum`
    /// - `loreum ${a.b.c} ipsum`
    /// - `loreum ${a} ipsum`
    fn is_plain_expression(&self) -> FormatResult<bool> {
        match self {
            TemplateElement::Js(template_element) => {
                let current_expression = template_element.expression()?;
                match current_expression {
                    JsAnyExpression::JsStaticMemberExpression(_)
                    | JsAnyExpression::JsComputedMemberExpression(_)
                    | JsAnyExpression::JsIdentifierExpression(_)
                    | JsAnyExpression::JsAnyLiteralExpression(_)
                    | JsAnyExpression::JsCallExpression(_)
                    | JsAnyExpression::JsParenthesizedExpression(_) => Ok(true),

                    _ => {
                        if let Some(function) =
                            JsAnyFunction::cast(current_expression.syntax().clone())
                        {
                            Ok(is_simple_function_expression(function)?)
                        } else {
                            Ok(false)
                        }
                    }
                }
            }
            TemplateElement::Ts(template_element) => {
                let is_mapped_type = matches!(template_element.ty()?, TsType::TsMappedType(_));
                Ok(!is_mapped_type)
            }
        }
    }

    fn has_comments(&self) -> bool {
        match self {
            TemplateElement::Js(template_element) => template_element.syntax().contains_comments(),
            TemplateElement::Ts(template_element) => template_element.syntax().contains_comments(),
        }
    }
}
