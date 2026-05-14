use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: generate_ast <output directory>");
        std::process::exit(64);
    }

    let output_dir: PathBuf = PathBuf::from(&args[1]);

    generate_ast(&output_dir)?;

    Ok(())
}

fn generate_ast(output_dir: &PathBuf) -> io::Result<()> {
    define_ast(
        output_dir,
        "Expr",
        vec![
            "Binary   : Box<Expr> left, Token operator, Box<Expr> right",
            "Grouping : Box<Expr> expression",
            "Literal  : Literal value",
            "Unary    : Token operator, Box<Expr> right",
        ],
    )
}

fn define_ast(output_dir: &PathBuf, base_name: &str, types: Vec<&str>) -> io::Result<()> {
    let output_file: PathBuf = output_dir.join(format!("expressions.rs",)); 
    let mut source: String = String::new();
    source.push_str("use crate::token::Token;\n");
    source.push_str("use crate::constants::Literal;\n");

    source.push_str(&format!("pub enum {} {{\n", base_name));
    for type_ in types {
        let parts: Vec<&str> = type_.split(":").collect();
        let class_name: &str = parts[0].trim();
        let fields: &str = parts[1].trim();
        source.push_str(&define_type(class_name, fields));
    }
    source.push_str("}");
    fs::write(output_file, source)
}

fn define_type(class_name: &str, field_list: &str) -> String {
    let mut source: String = String::new();
    source.push_str(&format!("    {} {{\n", class_name));
    for field in field_list.split(",") {
        let field_parts: Vec<&str> = field.trim().split(" ").collect();
        let field_type: &str = field_parts[0];
        let field_name: &str = field_parts[1];
        source.push_str(&format!("        {}: {},\n", field_name, field_type));
    }
    source.push_str("    },\n");
    source
}
