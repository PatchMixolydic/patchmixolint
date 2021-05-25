#![feature(rustc_private)]
#![feature(decl_macro)]
#![forbid(unsafe_op_in_unsafe_fn)]
#![warn(meta_variable_misuse)]

dylint_linting::dylint_library!();

extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_attr;
extern crate rustc_data_structures;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_hir_pretty;
extern crate rustc_index;
extern crate rustc_infer;
extern crate rustc_lexer;
extern crate rustc_lint;
extern crate rustc_lint_defs;
extern crate rustc_middle;
extern crate rustc_mir;
extern crate rustc_parse;
extern crate rustc_parse_format;
extern crate rustc_session;
extern crate rustc_span;
extern crate rustc_target;
extern crate rustc_trait_selection;
extern crate rustc_typeck;

mod macro_rules_over_macro;
mod missing_lints;
mod terse_lifetime_name;
mod utils;
mod impl_eq_for_float;

#[no_mangle]
pub fn register_lints(_sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_lints(&[
        macro_rules_over_macro::MACRO_RULES_OVER_MACRO,
        missing_lints::MISSING_LINTS,
        terse_lifetime_name::TERSE_LIFETIME_NAME,
        impl_eq_for_float::IMPL_EQ_FOR_FLOAT,
    ]);
    lint_store.register_early_pass(|| Box::new(macro_rules_over_macro::MacroRulesOverMacro));
    lint_store.register_early_pass(|| Box::new(missing_lints::MissingLints));
    lint_store.register_early_pass(|| Box::new(terse_lifetime_name::TerseLifetimeName));
    lint_store.register_late_pass(|| Box::new(impl_eq_for_float::ImplEqForFloat));
}

#[test]
fn ui() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui"),
    );
}
