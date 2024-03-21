use crate::tabi::value::Value;
use anyhow::bail;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

use super::ast::*;

#[derive(Parser)]
#[grammar = "./tabi/tabi.pest"]
struct TabiParser;

// Fun ideas:
// Have parser accept any string as identifier and validate here instead (better errors).
// This idea of a loose parser and a stricter weeding can be extended. Have it accept numbers with
// many dots and so on.

pub fn run(program: &String) -> anyhow::Result<Root> {
    // TODO: How to print a nice error?
    let mut parsed = TabiParser::parse(Rule::root, program)?;
    visit_root(parsed.next().unwrap())
}

///////////// AST BUILDERS /////////////
fn visit_root(root: Pair<'_, Rule>) -> anyhow::Result<Root> {
    // TODO: Check for overlapping function names.
    // Check that a main exists and has no arguments.
    let declarations = root
        .into_inner()
        .filter(|p| !matches!(p.as_rule(), Rule::EOI))
        .map(visit_declaration)
        .collect::<anyhow::Result<Vec<Declaration>>>()?;
    Ok(Root { declarations })
}

fn visit_declaration(declaration: Pair<'_, Rule>) -> anyhow::Result<Declaration> {
    match declaration.as_rule() {
        Rule::func => visit_func(declaration),
        other => bail!("Unsupported declaration {:?}", other),
    }
}

fn visit_func(func: Pair<'_, Rule>) -> anyhow::Result<Declaration> {
    let mut func = func.into_inner();
    let name = visit_ident(func.next().unwrap())?;
    let parameters = visit_parameters(func.next().unwrap())?;
    let return_type = visit_type(func.next().unwrap())?;
    let body = visit_block(func.next().unwrap())?;

    Ok(Declaration::Fn {
        name,
        parameters,
        return_type,
        body,
    })
}

fn visit_parameters(parameters: Pair<'_, Rule>) -> anyhow::Result<Vec<Parameter>> {
    // TODO: check that there is no overlapping names.
    parameters
        .into_inner()
        .map(visit_parameter)
        .collect::<anyhow::Result<Vec<Parameter>>>()
}

fn visit_parameter(parameter: Pair<'_, Rule>) -> anyhow::Result<Parameter> {
    let mut parameter = parameter.into_inner();
    let name = visit_ident(parameter.next().unwrap())?;
    let ttype = visit_type(parameter.next().unwrap())?;
    Ok(Parameter { name, ttype })
}

fn visit_expression(expression: Pair<'_, Rule>) -> anyhow::Result<Expression> {
    match expression.as_rule() {
        Rule::block => visit_block(expression),
        Rule::lowername => Ok(Expression::Ident(visit_ident(expression)?)),
        Rule::number | Rule::bool | Rule::string => {
            Ok(Expression::Literal(visit_literal(expression)?))
        }
        rule => bail!("Unexpected expression '{:?}'", rule),
    }
}

fn visit_block(block: Pair<'_, Rule>) -> anyhow::Result<Expression> {
    let expressions = block
        .into_inner()
        .map(visit_expression)
        .collect::<anyhow::Result<Vec<Expression>>>()?;
    Ok(Expression::Block { expressions })
}

fn visit_type(ttype: Pair<'_, Rule>) -> anyhow::Result<Type> {
    match ttype.as_rule() {
        Rule::uppername => Ok(Type::Ident(visit_ident(ttype)?)),
        rule => bail!("Unexpected type '{:?}'", rule),
    }
}

fn visit_literal(literal: Pair<'_, Rule>) -> anyhow::Result<Value> {
    match literal.as_rule() {
        Rule::bool => match literal.as_str() {
            "true" => Ok(Value::Bool(true)),
            "false" => Ok(Value::Bool(false)),
            _ => unreachable!(),
        },
        Rule::number => Ok(Value::Number(literal.as_str().parse::<f64>()?)),
        Rule::string => Ok(Value::String(literal.as_str().to_string())),
        rule => bail!("Unexpected literal '{:?}'", rule),
    }
}

fn visit_ident(ident: Pair<'_, Rule>) -> anyhow::Result<Ident> {
    match ident.as_rule() {
        Rule::lowername | Rule::uppername => Ok(Ident(ident.as_str().to_string())),
        rule => bail!("Unexpected ident '{:?}'", rule),
    }
}
