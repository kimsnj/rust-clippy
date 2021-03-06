// Copyright 2014-2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use crate::rustc::lint::{LateContext, LateLintPass, LintArray, LintPass};
use crate::rustc::{declare_tool_lint, lint_array};
use crate::rustc::hir::{Expr, ExprKind};
use crate::utils::{match_def_path, opt_def_id, paths, span_lint};

/// **What it does:** Checks for usage of `std::mem::forget(t)` where `t` is
/// `Drop`.
///
/// **Why is this bad?** `std::mem::forget(t)` prevents `t` from running its
/// destructor, possibly causing leaks.
///
/// **Known problems:** None.
///
/// **Example:**
/// ```rust
/// mem::forget(Rc::new(55)))
/// ```
declare_clippy_lint! {
    pub MEM_FORGET,
    restriction,
    "`mem::forget` usage on `Drop` types, likely to cause memory leaks"
}

pub struct MemForget;

impl LintPass for MemForget {
    fn get_lints(&self) -> LintArray {
        lint_array![MEM_FORGET]
    }
}

impl<'a, 'tcx> LateLintPass<'a, 'tcx> for MemForget {
    fn check_expr(&mut self, cx: &LateContext<'a, 'tcx>, e: &'tcx Expr) {
        if let ExprKind::Call(ref path_expr, ref args) = e.node {
            if let ExprKind::Path(ref qpath) = path_expr.node {
                if let Some(def_id) = opt_def_id(cx.tables.qpath_def(qpath, path_expr.hir_id)) {
                    if match_def_path(cx.tcx, def_id, &paths::MEM_FORGET) {
                        let forgot_ty = cx.tables.expr_ty(&args[0]);

                        if forgot_ty.ty_adt_def().map_or(false, |def| def.has_dtor(cx.tcx)) {
                            span_lint(cx, MEM_FORGET, e.span, "usage of mem::forget on Drop type");
                        }
                    }
                }
            }
        }
    }
}
