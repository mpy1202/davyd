use leptos::*;

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);

    
    view! {
        <div class="centre">
            <button
                on:click=move |_| {
                    // on stable, this is set_count.set(3);
                    set_count.update(|n| *n += 1);
                }
            >
                <h1>"Consider " {move || count()}</h1>            
                // on stable, this is move || count.get();
                
            </button>
        </div>
    }
}
fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|| view! { <App/> })
}  

//<div class="centre"><button></button></div> })