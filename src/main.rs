use leptos::*;
use std::*;

#[deny(warnings)]
#[allow(unused)]
#[allow(non_snake_case)]

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
    leptos::mount_to_body(App)
}


#[component]
fn App() -> impl IntoView {
    view! {
        <h2>"Controlled Component"</h2>
        <ControlledComponent/>
    }
}

#[component]
fn ControlledComponent() -> impl IntoView {

    let (expression, _set_expression) = create_signal("".to_string());
    let (result, set_result) = create_signal("".to_string());

    view! {
        <div>
            <input
                type="text"
                placeholder="Enter expression (e.g., 2 + 2)"
                value={expression.clone()}
                on:input=move |ev| {
                    set_result.set(evaluate_expression(&event_target_value(&ev).to_string()).unwrap().to_string());
                }
                prop:value=expression
            />
            //<button onclick={move || (expression.clone(), result.clone())}>Calculate</button>
            <div>{result}</div>
        </div>
    }

}