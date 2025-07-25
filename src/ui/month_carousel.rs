#![allow(non_snake_case)]
use chrono::{DateTime, Local};
use leptos::prelude::*;

#[component]
pub fn MonthCarousel() -> impl IntoView {
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let local: DateTime<Local> = Local::now();
    let (selected_month, set_selected_month) = signal(local.format("%B").to_string());
    view! {
        <div class="flex gap-2 overflow-x-auto pb-2">
            {months
                .into_iter()
                .map(|month| {
                    let month_str = month.to_string();
                    let month_for_button = month_str.clone();

                    view! {
                        <button
                            on:click=move |_| set_selected_month.set(month_for_button.clone())
                            class=move || {
                                if selected_month.get() == month_str {
                                    "px-3 py-1 rounded-full text-sm font-medium whitespace-nowrap transition-colors bg-green-600 text-white"
                                } else {
                                    "px-3 py-1 rounded-full text-sm font-medium whitespace-nowrap transition-colors bg-gray-100 text-gray-700 hover:bg-gray-200"
                                }
                            }
                        >
                            {month}
                        </button>
                    }
                })
                .collect::<Vec<_>>()}
        </div>
    }
}
