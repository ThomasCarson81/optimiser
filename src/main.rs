mod unusedfuncs;
use std::{env, fs};
use syntax::functions::CheckedProgram;
use unusedfuncs::remove_unused_functions;
enum OptimiserError {
    CouldNotReadFile(String),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a file to optimise.");
        return;
    }
    let mut program: CheckedProgram = match get_checked_program_from_file(&args[1]) {
        Ok(ast) => ast,
        Err(e) => {
            match e {
                OptimiserError::CouldNotReadFile(s) => {
                    println!("Error reading file: {}", s);
                }
            }
            std::process::exit(1);
        }
    };
    remove_unused_functions(&mut program.function_table, &program.main_function_id);

    if args.len() == 3 {
        output_optimised_program(&program, &args[2]);
    }
}

fn get_checked_program_from_file(file_path: &str) -> Result<CheckedProgram, OptimiserError> {
    let ast = std::fs::read_to_string(&file_path)
        .map_err(|e| OptimiserError::CouldNotReadFile(e.to_string()))?;
    let program: CheckedProgram =
        serde_json::from_str(&ast).map_err(|e| OptimiserError::CouldNotReadFile(e.to_string()))?;
    Ok(program)
}

fn output_optimised_program(program: &CheckedProgram, file_path: &str) {
    let json = serde_json::to_string_pretty(program);
    match json {
        Ok(json) => {
            fs::write(file_path, json).expect("Unable to write to file");
        }
        Err(e) => {
            println!("Error serialising checked AST JSON: {}", e);
            std::process::exit(1);
        }
    }
}
