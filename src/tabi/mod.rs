mod ast;
mod interpreter;
mod parser;
mod value;

pub fn run(file_name: String, program: String) -> anyhow::Result<value::Value> {
    let ast = parser::run(&program)?;
    let value = interpreter::visit_root(ast)?;
    Ok(value)
}
