use crate::components::text_input::TextInput;
use gloo::console::log;
use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    let binary_input_state = use_state(|| String::new());
    let binary_input_setter = binary_input_state.setter();

    let binary_input_changed = Callback::from(move |binary| {
        binary_input_setter.set(binary_to_decimal(&binary));
    });

    let decimal_input_state = use_state(|| String::new());
    let decimal_input_setter = decimal_input_state.setter();

    let decimal_input_changed = Callback::from(move |decimal| {
        decimal_input_setter.set(decimal_to_binary(&decimal));
    });

    html! {
        <main>
            <h2>{"Binary to Decimal"}</h2>
            <TextInput name="Binary Input" handle_onchange={binary_input_changed}/>
            <p>{&*binary_input_state}</p>
            <h2>{"Decimal to Binary"}</h2>
            <TextInput name="Decimal Input" handle_onchange={decimal_input_changed}/>
            <p>{&*decimal_input_state}</p>
        </main>
    }
}

fn binary_to_decimal(binary: &String) -> String {
    if let Ok(decimal) = isize::from_str_radix(binary, 2) {
        decimal.to_string()
    } else {
        String::from("Invalid Input.")
    }
}

fn decimal_to_binary(decimal: &String) -> String {
    if let Ok(decimal) = decimal.parse::<i64>() {
        format!("{:b}", decimal)
    } else {
        String::from("Invalid Input.")
    }
}
