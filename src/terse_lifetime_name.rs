use clippy_utils::diagnostics::span_lint_and_help;
use rustc_ast::{GenericParam, GenericParamKind};
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_lint_defs::declare_tool_lint;
use rustc_middle::lint::in_external_macro;
use rustc_session::declare_lint_pass;

declare_tool_lint! {
    /// **What it does:** Checks for lifetimes with names which are
    /// one character long.
    ///
    /// **Why is this bad?** The lifetime's name likely does not give
    /// any information about what it is used for.
    ///
    /// **Known problems:** Sometimes the lifetime's purpose is obvious
    /// or unimportant.
    ///
    /// **Example:**
    /// ```rust
    /// struct DiagnosticCtx<'a> {
    ///     source: &'a str,
    /// }
    /// ```
    /// Instead, use:
    /// ```rust
    /// struct DiagnosticCtx<'src> {
    ///     source: &'src str,
    /// }
    /// ```
    pub patchmixolint::TERSE_LIFETIME_NAME,
    Warn,
    "warns against single-character lifetime names"
}

declare_lint_pass!(TerseLifetimeName => [TERSE_LIFETIME_NAME]);

impl EarlyLintPass for TerseLifetimeName {
    fn check_generic_param(&mut self, ctx: &EarlyContext, param: &GenericParam) {
        if in_external_macro(ctx.sess, param.ident.span) {
            return;
        }

        if let GenericParamKind::Lifetime = param.kind {
            if !param.is_placeholder && param.ident.as_str().len() <= 2 {
                span_lint_and_help(
                    ctx,
                    TERSE_LIFETIME_NAME,
                    param.ident.span,
                    "single-character lifetime names are likely uninformative",
                    None,
                    "use a more informative name",
                );
            }
        }
    }
}
