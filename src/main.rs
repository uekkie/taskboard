use leptos::prelude::*;
use leptos::mount::mount_to_body;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(Counter)
}


#[component]
fn Counter() -> impl IntoView {
    let (count, set_count) = signal(0);
    let increment = move |_| set_count.set(count.get()+1);

    view! { 
        <button on:click=increment>{count}</button>
    }
}
