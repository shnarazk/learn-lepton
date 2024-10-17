use leptos::*;

fn main() {
    // mount_to_body(|| view! { <p>"Hello, world!"</p> })
    leptos::mount_to_body(|| view! { <App/> })
}

#[component]
fn App() -> impl IntoView {
    let (count, set_count) = create_signal(0);
    // let double_count = move || count.get() * 2;
    view! {
        <div class="grid grid-cols-3 gap-4 flex-row content-around bg-sky-500">
            <div class="m-5"
                class=("text-red-600", move || count.get() % 2 == 1)
                style=("font-size", move || format!("{}px", count.get() + 10))
           >
                {move || count.get()}
            </div>
            <div class="m-5">
                 none
            </div>
            <button
                class="primary underline m-4 border-2 border-red rounded-xl"
                style="display:inline-block;"
                on:click=move |_| {
                    // on stable, this is set_count.set(3);
                    set_count.update(|n| *n += 1);
                }
            >
                "Click me: "
                // on stable, this is move || count.get();
                {move || count.get()}
            </button>
        </div>
        <ProgressBar progress=count/>
    }
}

#[component]
fn ProgressBar(
    #[prop(default = 100)]
    max: u16,
    progress: ReadSignal<i32>
) -> impl IntoView {
    view! {
        <progress
          max=max
          value=progress
        />
    }
}