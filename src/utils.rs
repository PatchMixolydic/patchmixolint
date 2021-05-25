use rustc_ast::{ast, tokenstream::TokenTree, MacArgs};
use rustc_span::{sym, Symbol};

/// Check if a lint level is explicitly defined in the given crate's
/// top level attributes.
pub fn lint_level_declared(krate: &ast::Crate, sym: Symbol) -> bool {
    lint_level_declared_as(krate, sym, &[sym::allow, sym::warn, sym::deny, sym::forbid])
}

/// Check if a lint level is explicitly defined with one of the symbols in
/// `levels` in the given crate's top level attributes.
///
/// Note that passing the name of a non-lint-level attribute (like [`sym::feature`])
/// might cause false positives.
pub fn lint_level_declared_as(krate: &ast::Crate, sym: Symbol, levels: &[Symbol]) -> bool {
    krate.attrs.iter().any(|attr| {
        let is_lint_attr = levels.iter().any(|name| attr.has_name(*name));
        if !is_lint_attr {
            return false;
        }

        let is_lint_of_interest = match &attr.get_normal_item().args {
            MacArgs::Delimited(_, _, tokens) => tokens.trees().any(|token| match token {
                TokenTree::Token(token) => token.is_ident_named(sym),
                _ => false,
            }),

            _ => false,
        };

        is_lint_of_interest
    })
}
