use leptos::leptos_dom::logging::*;
use leptos::prelude::*;

#[component]
pub fn Counter() -> impl IntoView {
    console_log("Counter");
    let (value, set_value) = signal(0);

    Effect::new(move |_| {
        if value.get() >= 3 {
            set_value.set(0);
        }
    });

    view! {
        <button on:click=move |_| {
            console_log(&value.get().to_string());
            set_value.update(|value| *value += 1);
            console_log(&value.get().to_string());
        }>{value}</button>
    }
}

pub fn main() {
    mount_to_body(|| {
        view! { <Counter /> }
    })
}
