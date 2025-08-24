#![allow(non_snake_case)]
use crate::ui::growth::Growth;
use crate::ui::navbar::NavBar;
use crate::{api::growth::get_growths, ui::components::form_input_field::FormInputField};
use leptos::prelude::*;
use lucide_leptos::{
    CircleMinus, CirclePlus, Flower, Luggage, RulerDimensionLine, SearchCode, Trees,
};

#[component]
pub fn Divider() -> impl IntoView {
    view! { <div class="w-full h-px bg-gray-300"></div> }
}
#[component]
pub fn Selector<T, F1, F2>(
    options: Vec<T>,
    option_text: F1,
    option_value: F2,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] placeholder: Option<String>,
) -> impl IntoView
where
    T: Clone + 'static,
    F1: Fn(&T) -> String + 'static,
    F2: Fn(&T) -> String + 'static,
{
    view! {
        <div class="w-full">
            {label
                .map(|l| {
                    view! {
                        <label class="block text-sm font-medium text-gray-700 mb-1">{l}</label>
                    }
                })}
            <select class="w-full px-3 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none bg-white">
                {placeholder
                    .map(|p| {
                        view! {
                            <option value="" disabled selected>
                                {p}
                            </option>
                        }
                    })}
                {options
                    .into_iter()
                    .map(|option| {
                        let text = option_text(&option);
                        let value = option_value(&option);
                        view! { <option value=value>{text}</option> }
                    })
                    .collect::<Vec<_>>()}
            </select>
        </div>
    }
}

#[derive(Clone)]
struct User {
    id: u32,
    name: String,
}

#[component]
pub fn GrowthForm() -> impl IntoView {
    view! {
        <div class="bg-grey-400 flex flex-col gap-3 p-4 ease-in-out duration-300">
            <h>"New growth"</h>

            // <FormInputField placeholder="Search" icon=view! { <SearchCode /> } label="Plant type" />
            // TODO: Move options to struct
            <Selector
                options=vec![
                    "Tree".to_string(),
                    "Hedge".to_string(),
                    "Bush".to_string(),
                    "Potted".to_string(),
                ]
                option_text=|s: &String| s.clone()
                option_value=|s: &String| s.clone()
                label="Growth type"
            />
            // <FormInputField
            //     placeholder="Estimate"
            //     icon=view! { <Flower /> }
            //     label="Height estimate"
            // />
            // <FormInputField
            //     placeholder="Estimate"
            //     icon=view! { <RulerDimensionLine /> }
            //     label="Width estimate"
            // />

        </div>
    }
}

#[component]
pub fn Inventory() -> impl IntoView {
    let (form_open, set_form_open) = signal(false);

    // Create a resource that calls your server function
    let growths = Resource::new(|| (), |_| get_growths());

    view! {
        <div class="max-w-md mx-auto bg-gray-50 min-h-screen relative">
            <NavBar title="Growths".to_string()>
                <div></div>
            </NavBar>
            <div class="p-2">
                <div class="relative flex items-center gap-2">
                    <div class="flex-auto">
                        <div class="flex items-center">
                            <span class="absolute left-3 text-gray-400 flex items-center justify-center">
                                <SearchCode />
                            </span>
                            <input
                                type="text"
                                placeholder="Search your garden..."
                                class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none"
                            />
                        </div>
                    </div>

                    <Show when=move || { !form_open.get() }>
                        <div
                            class="border border-green-300 rounded-lg p-2 bg-green-300 cursor-pointer"
                            on:click=move |_| set_form_open.set(true)
                        >
                            <span class="text-white w-32">
                                <CirclePlus />
                            </span>
                        </div>
                    </Show>
                    <Show when=move || { form_open.get() }>
                        <div
                            class="border border-green-300 rounded-lg p-2 bg-green-300 cursor-pointer"
                            on:click=move |_| set_form_open.set(false)
                        >
                            <span class="text-white w-32">
                                <CircleMinus />
                            </span>
                        </div>
                    </Show>
                </div>
            </div>
            <Divider />
            // --- Input form
            <Show when=move || { form_open.get() }>
                <GrowthForm />
            </Show>
            // -- List of items
            <div class="w-full">
                <Suspense fallback=move || {
                    view! { <p>"Loading growths..."</p> }
                }>
                    {move || {
                        growths
                            .get()
                            .map(|result| match result {
                                Ok(growths) => {
                                    view! {
                                        {growths
                                            .iter()
                                            .map(|growth| {
                                                view! { <Growth growth=growth.to_owned() /> }
                                            })
                                            .collect::<Vec<_>>()}
                                    }
                                        .into_any()
                                }
                                Err(e) => {

                                    view! {
                                        <p class="error">
                                            "Error loading growths: " {e.to_string()}
                                        </p>
                                    }
                                        .into_any()
                                }
                            })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
