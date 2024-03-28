use leptos::*;

use std::{borrow::Borrow, cell::RefCell, rc::Rc};

#[deny(warnings)]
#[allow(unused)]
#[allow(non_snake_case)]




fn App() -> impl IntoView {
    let expression = State::new(String::new());
    let result = State::new(String::new());

    view! {
        <div>
            <input
                type="text"
                placeholder="Enter expression (e.g., 2 + 2)"
                value={expression.clone()}
                oninput={move |e: InputEvent| expression.set(e.value())}
            />
            <button onclick={move || calculate(expression.clone(), result.clone())}>Calculate</button>
            <div>{result.clone()}</div>
        </div>
    }
}

fn calculate(expression: State<String>, result: State<String>) {
    let expr = expression.get();
    let res = evaluate_expression(&expr);
    match res {
        Ok(value) => result.set(format!("Result: {}", value)),
        Err(err) => result.set(format!("Error: {}", err)),
    }
}

pub fn evaluate_expression(expr: &str) -> Result<f64, String> {
    let parts: Vec<&str> = expr.split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Invalid expression".to_string());
    }

    let num1: f64 = match parts[0].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid number".to_string()),
    };

    let operator = parts[1];
    let num2: f64 = match parts[2].parse() {
        Ok(num) => num,
        Err(_) => return Err("Invalid number".to_string()),
    };

    match operator {
        "+" => Ok(num1 + num2),
        "-" => Ok(num1 - num2),
        "*" => Ok(num1 * num2),
        "/" => {
            if num2 != 0.0 {
                Ok(num1 / num2)
            } else {
                Err("Division by zero".to_string())
            }
        }
        _ => Err("Invalid operator".to_string()),
    }
}



fn main() {
    mount_to_body(move || {
        view! { <App/> }
    });
}