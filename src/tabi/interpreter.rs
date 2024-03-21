use std::collections::HashMap;

use anyhow::bail;

use super::{ast::*, value::Value};

struct Context {
    inner: HashMap<String, Value>,
}

impl Context {
    fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    fn look_up(&self, key: &String) -> anyhow::Result<Value> {
        match self.inner.get(key) {
            Some(value) => Ok(value.clone()), // TODO: This clone can be expensive.
            None => bail!("'{key}' is undefined here."),
        }
    }

    fn assign(&mut self, key: &str, value: Value) -> anyhow::Result<()> {
        match self.inner.insert(key.to_string(), value) {
            None => Ok(()),
            Some(_) => bail!("Cannot reassign '{key}'."),
        }
    }
}

pub fn visit_root(tree: Root) -> anyhow::Result<Value> {
    // TODO: make context with all declarations in it.
    // TODO: Find and run main
    match tree.declarations.get(0).unwrap() {
        Declaration::Fn {
            name: _,
            parameters: _,
            return_type: _,
            body,
        } => visit_expression(Context::new(), body),
    }
}

fn visit_expression(context: Context, expression: &Expression) -> anyhow::Result<Value> {
    match expression {
        Expression::Ident(Ident(ident)) => context.look_up(&ident),
        Expression::Literal(value) => Ok(value.clone()),
        Expression::Block { expressions } => visit_expression(context, expressions.get(0).unwrap()), // TODO: recursive fold so context gets updated!
                                                                                                     //
    }
}
