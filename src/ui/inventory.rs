#![allow(non_snake_case)]
use crate::api::growth::get_growths;
use crate::ui::growth::Growth;
use crate::ui::navbar::NavBar;
use leptos::prelude::*;

#[component]
pub fn Inventory() -> impl IntoView {
    // Create a resource that calls your server function
    let growths = Resource::new(|| (), |_| get_growths());

    view! {
        <div class="max-w-md mx-auto bg-gray-50 min-h-screen relative">
            <NavBar title="Inventory".to_string()>
                <div></div>
            </NavBar>
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
