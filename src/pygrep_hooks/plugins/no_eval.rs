use rustpython_ast::{Expr, ExprKind};

use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::Check;
use crate::violations;

/// PGH001 - no eval
pub fn no_eval(checker: &mut Checker, func: &Expr) {
    let ExprKind::Name { id, .. } = &func.node else {
        return;
    };
    if id != "eval" {
        return;
    }
    if !checker.is_builtin("eval") {
        return;
    }
    checker
        .checks
        .push(Check::new(violations::NoEval, Range::from_located(func)));
}
