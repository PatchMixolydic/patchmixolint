use itertools::Itertools;
use rustc_ast::{ast, token::{Token, TokenKind}, tokenstream::TokenTree, MacArgs};
use rustc_span::{sym, Symbol};

/// Check if a lint level is explicitly defined in the given crate's
/// top level attributes.
pub fn lint_level_declared(krate: &ast::Crate, lint: Symbol) -> bool {
    lint_level_declared_as(krate, lint, &[sym::allow, sym::warn, sym::deny, sym::forbid])
}

/// Check if a lint level is explicitly defined with one of the symbols in
/// `levels` in the given crate's top level attributes.
///
/// Note that passing the name of a non-lint-level attribute (like [`sym::feature`])
/// might cause false positives.
pub fn lint_level_declared_as(krate: &ast::Crate, lint: Symbol, levels: &[Symbol]) -> bool {
    krate.attrs.iter().any(|attr| {
        let is_lint_attr = levels.iter().any(|name| attr.has_name(*name));
        if !is_lint_attr {
            return false;
        }

        match &attr.get_normal_item().args {
            MacArgs::Delimited(_, _, tokens) => tokens.trees().any(|token| match token {
                TokenTree::Token(token) => token.is_ident_named(lint),
                _ => false,
            }),

            _ => false,
        }
    })
}

/// Check if a lint level is explicitly defined in the given crate's
/// top level attributes.
pub fn tool_lint_level_declared(krate: &ast::Crate, tool: Symbol, lint: Symbol) -> bool {
    tool_lint_level_declared_as(krate, tool, lint, &[sym::allow, sym::warn, sym::deny, sym::forbid])
}

/// Check if a lint level is explicitly defined with one of the symbols in
/// `levels` in the given crate's top level attributes.
///
/// Note that passing the name of a non-lint-level attribute (like [`sym::feature`])
/// might cause false positives.
pub fn tool_lint_level_declared_as(
    krate: &ast::Crate,
    target_tool: Symbol,
    target_lint: Symbol,
    levels: &[Symbol],
) -> bool {
    krate.attrs.iter().any(|attr| {
        let is_lint_attr = levels.iter().any(|name| attr.has_name(*name));
        if !is_lint_attr {
            return false;
        }

        match &attr.get_normal_item().args {
            MacArgs::Delimited(_, _, tokens) => tokens
                .trees()
                .tuple_windows::<(TokenTree, TokenTree, TokenTree)>()
                .any(|tokens| match tokens {
                    (
                        TokenTree::Token(Token {
                            kind: TokenKind::Ident(tool_id, _), ..
                        }),
                        TokenTree::Token(Token {
                            kind: TokenKind::ModSep, ..
                        }),
                        TokenTree::Token(Token {
                            kind: TokenKind::Ident(lint_id, _), ..
                        }),
                    ) => {
                        tool_id == target_tool && lint_id == target_lint
                    }

                    _ => false,
                }),

            _ => false,
        }
    })
}
