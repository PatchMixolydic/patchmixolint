use if_chain::if_chain;
use rustc_hir::{def_id::LOCAL_CRATE, Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_lint_defs::declare_tool_lint;
use rustc_middle::{
    lint::in_external_macro,
    ty::{Ty, TyKind},
};
use rustc_session::declare_lint_pass;
use rustc_span::{sym, symbol::Ident, Span};

declare_tool_lint! {
    /// **What it does:** Checks for implementations of `Eq` on types
    /// that contain floats or other types that contain floats.
    ///
    /// **Why is this bad?** `f32` and `f64` do not implement `Eq` on
    /// purpose because `NaN != NaN`.
    ///
    /// **Known problems:** This lint does not check types that come
    /// from other crates to avoid linting types such as `noisy_float`'s
    /// `R32`. This lint may trip for types with safe implementations of `Eq`
    /// (eg. manual implementations of `noisy_float`'s types) and for
    /// implementations of `Eq` on types that contain such types.
    ///
    /// **Example:**
    /// ```rust
    /// struct MyFloat {
    ///     x: f32,
    /// }
    ///
    /// impl Eq for MyFloat {}
    /// ```
    /// Instead, use:
    /// ```rust
    /// use noisy_float::prelude::*;
    ///
    /// #[derive(Eq)]
    /// struct MyFloat {
    ///     x: R32,
    /// }
    /// ```
    pub patchmixolint::IMPL_EQ_FOR_FLOAT,
    Deny,
    "lints against `impl Eq for T` when `T` contains a float type"
}

declare_lint_pass!(ImplEqForFloat => [IMPL_EQ_FOR_FLOAT]);

fn emit_eq_for_float_err<'tcx>(
    ctx: &LateContext<'tcx>,
    impl_span: Span,
    maybe_ty_def_ident: Option<Ident>,
    fields: Vec<(Ident, Ty)>,
) {
    ctx.struct_span_lint(IMPL_EQ_FOR_FLOAT, impl_span, |diag| {
        let mut diag = diag.build("`Eq` should not be implemented for types containing floats");

        if let Some(ty_def_ident) = maybe_ty_def_ident {
            diag.span_label(
                impl_span,
                format!("`{}` should not impl `Eq`", ty_def_ident),
            )
            .span_label(
                ty_def_ident.span,
                format!("`{}` defined here", ty_def_ident),
            );
        }

        for (ident, ty) in fields {
            let extra_info = match ty.kind() {
                TyKind::Adt(..) => ", which contains a float",
                _ => "",
            };

            diag.span_label(
                ident.span,
                format!(
                    "`{}` is of type {}{}",
                    ident,
                    ty.sort_string(ctx.tcx),
                    extra_info,
                ),
            );
        }

        diag.note("floats do not implement `Eq` since `NaN` is not equal to itself")
            .help("consider using a crate such as `noisy_float` or `decorum`")
            .emit();
    });
}

fn has_float_ty<'tcx>(ctx: &LateContext<'tcx>, ty: Ty<'tcx>) -> bool {
    match ty.kind() {
        TyKind::Float(_) => true,

        TyKind::Adt(adt, substs) => {
            if adt.did.krate != LOCAL_CRATE {
                // don't lint against items in other crates
                return false;
            }

            for field in adt.all_fields() {
                let ty = field.ty(ctx.tcx, substs);

                if has_float_ty(ctx, ty) {
                    return true;
                }
            }

            false
        }

        _ => false,
    }
}

impl<'tcx> LateLintPass<'tcx> for ImplEqForFloat {
    fn check_item(&mut self, ctx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        if in_external_macro(ctx.tcx.sess, item.span) {
            return;
        }

        if_chain! {
            if let ItemKind::Impl(imp) = &item.kind;
            if let Some(trait_ref) = &imp.of_trait;
            // Unfortunately, `Eq` has no lang item.
            // Hope that nobody defines another `Eq` trait.
            if let Some(last_seg) = trait_ref.path.segments.last();
            if last_seg.ident.name == sym::Eq;

            // This is an `Eq` impl. Does the item it's attached to
            // contain a float type?
            if let TyKind::Adt(adt, substs) = ctx.tcx.type_of(imp.self_ty.hir_id.owner).kind();
            then {
                let mut float_fields = Vec::new();

                for field in adt.all_fields() {
                    let ty = field.ty(ctx.tcx, substs);

                    if has_float_ty(ctx, ty) {
                        float_fields.push((field.ident, ty));
                    }
                }

                if !float_fields.is_empty() {
                    emit_eq_for_float_err(
                        ctx,
                        item.span,
                        ctx.tcx.opt_item_name(adt.did),
                        float_fields
                    );
                }
            }
        }
    }
}
