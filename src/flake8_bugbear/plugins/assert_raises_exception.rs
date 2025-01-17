use rustpython_ast::{ExprKind, Stmt, Withitem};

use crate::ast::helpers::match_module_member;
use crate::ast::types::Range;
use crate::checkers::ast::Checker;
use crate::registry::Check;
use crate::violations;

/// B017
pub fn assert_raises_exception(checker: &mut Checker, stmt: &Stmt, items: &[Withitem]) {
    let Some(item) = items.first() else {
        return;
    };
    let item_context = &item.context_expr;
    let ExprKind::Call { func, args, .. } = &item_context.node else {
        return;
    };
    if args.len() != 1 {
        return;
    }
    if item.optional_vars.is_some() {
        return;
    }
    if !matches!(&func.node, ExprKind::Attribute { attr, .. } if attr == "assertRaises") {
        return;
    }
    if !match_module_member(
        args.first().unwrap(),
        "",
        "Exception",
        &checker.from_imports,
        &checker.import_aliases,
    ) {
        return;
    }

    checker.checks.push(Check::new(
        violations::NoAssertRaisesException,
        Range::from_located(stmt),
    ));
}
