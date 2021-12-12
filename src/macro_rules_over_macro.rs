use clippy_utils::diagnostics::span_lint_and_sugg;
use rustc_ast::ast;
use rustc_errors::Applicability;
use rustc_lint::{EarlyContext, EarlyLintPass};
use rustc_lint_defs::declare_tool_lint;
use rustc_middle::lint::in_external_macro;
use rustc_session::declare_lint_pass;

declare_tool_lint! {
    /// **What it does:** Checks for uses of `macro_rules` when the
    /// `decl_macro` feature enabled.
    ///
    /// **Why is this bad?** `macro` is more consistent with respect
    /// to privacy and should be preferred.
    ///
    /// **Known problems:** `macro` macros have different hygiene rules,
    /// which means the suggested change can break working macros.
    ///
    /// **Example:**
    /// ```rust
    /// #![feature(decl_macro)]
    /// #![feature(never_type)]
    ///
    /// macro_rules! foo {
    ///     (become unsafe) => {
    ///         *(1 as *const !)
    ///     };
    /// }
    /// ```
    /// Instead, use:
    /// ```rust
    /// #![feature(decl_macro)]
    /// #![feature(never_type)]
    ///
    /// macro foo(become unsafe) {
    ///     *(1 as *const !)
    /// }
    /// ```
    pub patchmixolint::MACRO_RULES_OVER_MACRO,
    Warn,
    "warns against the use of `macro_rules!` when `decl_macro` is enabled"
}

declare_lint_pass!(MacroRulesOverMacro => [MACRO_RULES_OVER_MACRO]);

impl EarlyLintPass for MacroRulesOverMacro {
    fn check_item(&mut self, ctx: &EarlyContext<'_>, item: &ast::Item) {
        // Don't lint if `decl_macro` isn't enabled, and don't lint against `macro_rules!`
        // definitions generated by external macros
        if !ctx.sess.features_untracked().decl_macro || in_external_macro(ctx.sess, item.span) {
            return;
        }

        let macro_def = match &item.kind {
            rustc_ast::ItemKind::MacroDef(def) => (def),
            _ => return,
        };

        if !macro_def.macro_rules {
            // this macro already uses `macro`
            return;
        }

        let macro_rules_span = item.span.shrink_to_lo().until(item.ident.span);

        span_lint_and_sugg(
            ctx,
            MACRO_RULES_OVER_MACRO,
            macro_rules_span,
            "`macro_rules!` was used, but the `decl_macro` feature is enabled",
            "use",
            "macro ".into(),
            // might cause hygiene issues
            Applicability::MaybeIncorrect,
        );
    }
}
