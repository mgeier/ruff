use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHashSet};
use rustpython_ast::{Excepthandler, ExcepthandlerKind, Expr, ExprContext, ExprKind, Location};

use crate::ast::helpers;
use crate::ast::types::Range;
use crate::autofix::Fix;
use crate::checkers::ast::Checker;
use crate::registry::{Check, CheckCode};
use crate::source_code_generator::SourceCodeGenerator;
use crate::violations;

fn type_pattern(elts: Vec<&Expr>) -> Expr {
    Expr::new(
        Location::default(),
        Location::default(),
        ExprKind::Tuple {
            elts: elts.into_iter().cloned().collect(),
            ctx: ExprContext::Load,
        },
    )
}

fn duplicate_handler_exceptions<'a>(
    checker: &mut Checker,
    expr: &'a Expr,
    elts: &'a [Expr],
) -> FxHashMap<Vec<&'a str>, &'a Expr> {
    let mut seen: FxHashMap<Vec<&str>, &Expr> = FxHashMap::default();
    let mut duplicates: FxHashSet<Vec<&str>> = FxHashSet::default();
    let mut unique_elts: Vec<&Expr> = Vec::default();
    for type_ in elts {
        let call_path = helpers::collect_call_paths(type_);
        if !call_path.is_empty() {
            if seen.contains_key(&call_path) {
                duplicates.insert(call_path);
            } else {
                seen.entry(call_path).or_insert(type_);
                unique_elts.push(type_);
            }
        }
    }

    if checker.settings.enabled.contains(&CheckCode::B014) {
        // TODO(charlie): Handle "BaseException" and redundant exception aliases.
        if !duplicates.is_empty() {
            let mut check = Check::new(
                violations::DuplicateHandlerException(
                    duplicates
                        .into_iter()
                        .map(|call_path| call_path.join("."))
                        .sorted()
                        .collect::<Vec<String>>(),
                ),
                Range::from_located(expr),
            );
            if checker.patch(check.kind.code()) {
                let mut generator: SourceCodeGenerator = checker.style.into();
                if unique_elts.len() == 1 {
                    generator.unparse_expr(unique_elts[0], 0);
                } else {
                    generator.unparse_expr(&type_pattern(unique_elts), 0);
                }
                check.amend(Fix::replacement(
                    generator.generate(),
                    expr.location,
                    expr.end_location.unwrap(),
                ));
            }
            checker.checks.push(check);
        }
    }

    seen
}

pub fn duplicate_exceptions(checker: &mut Checker, handlers: &[Excepthandler]) {
    let mut seen: FxHashSet<Vec<&str>> = FxHashSet::default();
    let mut duplicates: FxHashMap<Vec<&str>, Vec<&Expr>> = FxHashMap::default();
    for handler in handlers {
        let ExcepthandlerKind::ExceptHandler { type_: Some(type_), .. } = &handler.node else {
            continue;
        };
        match &type_.node {
            ExprKind::Attribute { .. } | ExprKind::Name { .. } => {
                let call_path = helpers::collect_call_paths(type_);
                if !call_path.is_empty() {
                    if seen.contains(&call_path) {
                        duplicates.entry(call_path).or_default().push(type_);
                    } else {
                        seen.insert(call_path);
                    }
                }
            }
            ExprKind::Tuple { elts, .. } => {
                for (name, expr) in duplicate_handler_exceptions(checker, type_, elts) {
                    if seen.contains(&name) {
                        duplicates.entry(name).or_default().push(expr);
                    } else {
                        seen.insert(name);
                    }
                }
            }
            _ => {}
        }
    }

    if checker.settings.enabled.contains(&CheckCode::B025) {
        for (name, exprs) in duplicates {
            for expr in exprs {
                checker.checks.push(Check::new(
                    violations::DuplicateTryBlockException(name.join(".")),
                    Range::from_located(expr),
                ));
            }
        }
    }
}
