use symbolica::parser::parse;
use symbolica::symbolic::Symbolic;
use std::collections::HashSet;
use std::io::{self, Write};

fn solve_any_number_of_equations() -> Result<symbolica::symbolic::Solution, symbolica::error::Error> {
    let mut equations_str = vec![];

    loop {
        print!("Enter an equation (or press Enter to finish): ");
        io::stdout().flush().unwrap();
        let mut eq = String::new();
        io::stdin().read_line(&mut eq).unwrap();
        let eq = eq.trim();
        if eq.is_empty() {
            break;
        }
        equations_str.push(eq.to_string());
    }

    let equations: Vec<Symbolic> = equations_str.iter()
        .map(|eq| {
            let sides: Vec<&str> = eq.split('=').collect();
            let lhs = parse(sides[0]).unwrap();
            let rhs = parse(sides[1]).unwrap();
            lhs - rhs
        })
        .collect();

    let variables: HashSet<_> = equations.iter()
        .flat_map(|eq| eq.free_symbols())
        .collect();

    symbolica::solve::solve_equations(equations, &variables.into_iter().collect::<Vec<_>>())
}

fn main() {
    match solve_any_number_of_equations() {
        Ok(solutions) => println!("Solutions: {:?}", solutions),
        Err(err) => eprintln!("An error occurred: {:?}", err),
    }
}
