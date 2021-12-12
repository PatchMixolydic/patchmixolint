use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::ast;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_lint_defs::declare_tool_lint;
use rustc_session::declare_lint_pass;
use rustc_span::{sym, Symbol};
use std::borrow::Cow;

use crate::utils::{lint_level_declared, lint_level_declared_as, tool_lint_level_declared};

declare_tool_lint! {
    /// **What it does:** Checks for missing lint level declarations.
    ///
    /// **Why is this bad?** Several useful lints are allowed by default,
    /// such as `meta_variable_misuse` and `unsafe_op_in_unsafe_fn`.
    ///
    /// **Known problems:** This lint can be incredibly noisy and might
    /// suggest setting lint levels which are irrelevant to your crate.
    ///
    /// **Example:**
    /// ```rust
    /// #![forbid(macro_use_extern_crate)]
    ///
    /// fn main() {}
    /// ```
    /// Instead, use:
    /// ```rust
    /// #![forbid(macro_use_extern_crate)]
    /// #![forbid(unsafe_op_in_unsafe_fn)]
    /// #![warn(meta_variable_misuse)]
    ///
    /// fn main() {}
    /// ```
    pub patchmixolint::MISSING_LINTS,
    Warn,
    "warns if certain lint levels aren't explicitly declared"
}

declare_lint_pass!(MissingLints => [MISSING_LINTS]);

fn lint_on_undeclared_level(
    ctx: &EarlyContext<'_>,
    krate: &ast::Crate,
    name: &str,
    suggest_level: &str,
) {
    if !lint_level_declared(krate, Symbol::intern(name)) {
        let span = krate
            .span
            .shrink_to_lo()
            .until(krate.attrs.last().map(|attr| attr.span).unwrap_or_default());

        let location = if let Some(crate_name) = &ctx.sess.opts.crate_name {
            Cow::Owned(format!(" in crate `{}`", crate_name))
        } else {
            Cow::Borrowed("")
        };

        span_lint_and_help(
            ctx,
            MISSING_LINTS,
            span,
            &format!("missing lint level for `{}`{}", name, location),
            None,
            &format!(
                "declare the lint level explicitly: `#![{}({})]`",
                suggest_level, name
            ),
        );
    }
}

fn lint_on_undeclared_tool_level(
    ctx: &EarlyContext<'_>,
    krate: &ast::Crate,
    tool: Symbol,
    name: &str,
    suggest_level: &str,
) {
    if !tool_lint_level_declared(krate, tool, Symbol::intern(name)) {
        let span = krate
            .span
            .shrink_to_lo()
            .until(krate.attrs.last().map(|attr| attr.span).unwrap_or_default());

        let location = if let Some(crate_name) = &ctx.sess.opts.crate_name {
            Cow::Owned(format!(" in crate `{}`", crate_name))
        } else {
            Cow::Borrowed("")
        };

        span_lint_and_help(
            ctx,
            MISSING_LINTS,
            span,
            &format!("missing lint level for `{}::{}`{}", tool, name, location),
            None,
            &format!(
                "declare the lint level explicitly: `#![{}({}::{})]`",
                suggest_level, tool, name
            ),
        );
    }
}

impl EarlyLintPass for MissingLints {
    fn check_crate(&mut self, ctx: &EarlyContext<'_>, krate: &ast::Crate) {
        lint_on_undeclared_level(ctx, krate, "meta_variable_misuse", "warn");

        if !lint_level_declared_as(
            krate,
            Symbol::intern("unsafe_code"),
            &[sym::deny, sym::forbid],
        ) {
            lint_on_undeclared_level(ctx, krate, "unsafe_op_in_unsafe_fn", "forbid");
            lint_on_undeclared_tool_level(
                ctx,
                krate,
                sym::clippy,
                "undocumented_unsafe_blocks",
                "forbid",
            );
        }

        if lint_level_declared_as(krate, Symbol::intern("unused"), &[sym::allow]) {
            lint_on_undeclared_level(ctx, krate, "unused_imports", "warn");
            lint_on_undeclared_level(ctx, krate, "unused_must_use", "warn");
        }
    }
}
