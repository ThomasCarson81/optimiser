use std::collections::HashSet;

use syntax::{
    ast::{Expression, Statement},
    functions::{FunctionBody, FunctionId, FunctionTable},
};

pub fn remove_unused_functions(function_table: &mut FunctionTable, main_id: &FunctionId) {
    let mut used: HashSet<FunctionId> = HashSet::new();
    explore_function(main_id, function_table, &mut used);

    function_table
        .functions
        .retain(|function| used.contains(&function.id));
}

fn explore_function(
    function_id: &FunctionId,
    function_table: &FunctionTable,
    used: &mut HashSet<FunctionId>,
) {
    if used.contains(&function_id) {
        return;
    }

    used.insert(function_id.clone());

    if let Some(function) = function_table.get(function_id) {
        match &function.implementation {
            syntax::functions::FunctionImplementation::Oxide(body) => {
                explore_function_body(body.clone(), function_table, used);
            }
            syntax::functions::FunctionImplementation::Builtin(_) => {
                used.insert(function_id.clone());
            }
        }
    }
}

fn explore_function_body(
    body: FunctionBody,
    function_table: &FunctionTable,
    used: &mut HashSet<FunctionId>,
) {
    for statement in &body.statements {
        explore_statement(statement, function_table, used);
    }
}

fn explore_statement(
    statement: &Statement,
    function_table: &FunctionTable,
    used: &mut HashSet<FunctionId>,
) {
    match statement {
        Statement::Let { expression, .. } => explore_expression(expression, function_table, used),
        Statement::Assignment { expression, .. } => {
            explore_expression(expression, function_table, used)
        }
        Statement::If {
            condition,
            body,
            else_block,
        } => {
            explore_expression(condition, function_table, used);
            for stat in body {
                explore_statement(stat, function_table, used);
            }
            if let Some(else_body) = else_block {
                for stat in else_body {
                    explore_statement(stat, function_table, used);
                }
            }
        }
        Statement::While { condition, body } => {
            explore_expression(condition, function_table, used);
            for stat in body {
                explore_statement(stat, function_table, used);
            }
        }
        Statement::For {
            initialiser,
            condition,
            increment,
            body,
        } => {
            if let Some(init) = initialiser {
                explore_statement(init, function_table, used);
            }
            if let Some(cond) = condition {
                explore_expression(cond, function_table, used);
            }
            if let Some(inc) = increment {
                explore_statement(inc, function_table, used);
            }
            for stat in body {
                explore_statement(stat, function_table, used);
            }
        }
        Statement::Scope { body } => {
            for stat in body {
                explore_statement(stat, function_table, used);
            }
        }
        Statement::FunctionDefinition { body, .. } => {
            for stat in body {
                explore_statement(stat, function_table, used);
            }
        }
        Statement::ExpressionStatement { expression } => {
            explore_expression(expression, function_table, used);
        }
        Statement::Return { value } => {
            if let Some(expr) = value {
                explore_expression(expr, function_table, used);
            }
        }
        Statement::Break => {}
        Statement::Continue => {}
        Statement::Pass => {}
        Statement::TypeDefinition(_) => todo!(),
    }
}

fn explore_expression(
    expression: &Expression,
    function_table: &FunctionTable,
    used: &mut HashSet<FunctionId>,
) {
    match expression {
        Expression::ResolvedFunctionCall {
            function_id,
            arguments,
        } => {
            for arg in arguments {
                explore_expression(arg, function_table, used);
            }
            explore_function(function_id, function_table, used);
        }
        Expression::BinaryOp {
            left,
            operator: _,
            right,
            inferred_type: _,
        } => {
            explore_expression(left, function_table, used);
            explore_expression(right, function_table, used);
        }
        Expression::UnaryOp {
            operator: _,
            operand,
            inferred_type: _,
        } => {
            explore_expression(operand, function_table, used);
        }
        Expression::Identifier(_) | Expression::Literal(_) => {}
        Expression::UnresolvedFunctionCall { .. } => {
            unreachable!("Unresolved function calls should not be present in the checked AST")
        }
    }
}
