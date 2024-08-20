use symengine::{Expression, Symbol, solve, Eq, Set};
use std::collections::HashSet;
use std::io::{self, Write};

fn main() {
    let mut equations_str = Vec::new();
    
    loop {
        print!("Enter an equation (or press Enter to finish): ");
        io::stdout().flush().unwrap();  // Make sure the prompt is displayed
        let mut eq = String::new();
        io::stdin().read_line(&mut eq).unwrap();
        let eq = eq.trim();
        
        if eq.is_empty() {
            break;
        }
        
        equations_str.push(eq.to_string());
    }

    let mut equations = Vec::new();
    let mut variables = HashSet::new();

    for eq_str in equations_str {
        let parts: Vec<&str> = eq_str.split('=').collect();
        if parts.len() != 2 {
            eprintln!("Invalid equation format: {}", eq_str);
            continue;
        }

        let lhs = Expression::from(parts[0].trim());
        let rhs = Expression::from(parts[1].trim());
        let equation = Eq(lhs - rhs, Expression::from(0));

        equations.push(equation);

        // Collect variables
        let mut eq_vars = lhs.free_symbols();
        eq_vars.extend(rhs.free_symbols());
        for var in eq_vars {
            variables.insert(var);
        }
    }

    let symbols: Vec<Symbol> = variables.into_iter().collect();
    let solutions = solve(&equations, &symbols);

    println!("Solutions: {:?}", solutions);
}
