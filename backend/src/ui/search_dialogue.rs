use std::str::FromStr;

use crate::ui::components::badge::Badge;
use crate::ui::components::form_input_field::FormInputField;
use entity::search::{PlantRecord, SearchResults};
use leptos::ev;
use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use leptos_router::hooks::use_location;
use lucide_leptos::{ExternalLink, House, Leaf, Menu, Plus, Search, Sprout, X};

#[component]
pub fn SearchContent() -> impl IntoView {
    let search_categories = vec!["Plants"];
    let plant_results = SearchResults {
        plants: vec![PlantRecord {
            common_name: "Rose".to_string(),
            family: "Rose".to_string(),
            description: "Some description from wiki".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/e/e6/Rosa_rubiginosa_1.jpg/1024px-Rosa_rubiginosa_1.jpg"
                .to_string(),
            wiki_url: "https://en.wikipedia.org/wiki/Rose".to_string(),
            in_garden: true
        }, PlantRecord {
            common_name: "Rose".to_string(),
            family: "Rose".to_string(),
            description: "Some description from wiki".to_string(),
            image_url: "https://upload.wikimedia.org/wikipedia/commons/thumb/e/e6/Rosa_rubiginosa_1.jpg/1024px-Rosa_rubiginosa_1.jpg"
                .to_string(),
            wiki_url: "https://en.wikipedia.org/wiki/Rose".to_string(),
            in_garden: false
        }],
    };
    view! {
        <div class="overflow-y-auto max-h-[50vh] space-y-6">
            {search_categories
                .into_iter()
                .map(|category| {
                    let category_str = category.to_string();
                    view! {
                        <div class="space-y-3">
                            <h3 class="text-lg font-semibold text-foreground border-b border-border pb-2">
                                {category_str.clone()}
                            </h3>
                            <div class="space-y-2">
                                {plant_results
                                    .plants
                                    .clone()
                                    .into_iter()
                                    .map(|plant| {

                                        view! {
                                            <div class="flex gap-4 p-4 bg-card bg-green-50 rounded-lg hover:shadow-md transition-shadow">
                                                <div class="flex flex-col gap-2 flex-shrink-0">
                                                    <img
                                                        src=plant.image_url.to_string()
                                                        alt=plant.common_name.to_string()
                                                        class="w-20 h-20 rounded-lg object-cover"
                                                    />
                                                    <Badge>

                                                        {plant.family.to_string()}
                                                    </Badge>
                                                </div>

                                                <div class="flex-1 min-w-0 flex flex-col justify-between">
                                                    <div>
                                                        <div class="flex items-center gap-3 mb-2">
                                                            <h4 class="font-medium text-card-foreground">
                                                                {plant.common_name}
                                                            </h4>

                                                        </div>
                                                        <p class="text-muted-foreground mb-3 line-clamp-2 text-xs">
                                                            {plant.description}
                                                        </p>
                                                    </div>

                                                    <div class="flex items-center justify-between">
                                                        <a
                                                            href=plant.wiki_url.to_string()
                                                            target="_blank"
                                                            rel="noopener noreferrer"
                                                            class="inline-flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors"
                                                        >
                                                            Wiki
                                                            <ExternalLink size=10 />
                                                        </a>

                                                        <Show
                                                            when=move || plant.in_garden
                                                            fallback=move || {
                                                                view! {
                                                                    <Badge>
                                                                        <div class="flex flex-row gap-1 items-center cursor-pointer">
                                                                            <Plus size=10 />
                                                                            <p>Add to Garden</p>
                                                                        </div>
                                                                    </Badge>
                                                                }
                                                            }
                                                        >

                                                            <Badge>
                                                                // variant="secondary"
                                                                // class="bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200"
                                                                In Garden
                                                            </Badge>
                                                        </Show>
                                                    </div>
                                                </div>
                                            </div>
                                        }
                                    })
                                    .collect::<Vec<_>>()}
                            </div>
                        </div>
                    }
                })
                .collect::<Vec<_>>()}

        </div>
    }
}

#[component]
pub fn SearchDialog(
    search_input: ReadSignal<String>,
    set_search_input: WriteSignal<String>,
) -> impl IntoView {
    let prevent_close = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
    };
    // let async_data = LocalResource::new(move || load_data(search_input.get()));
    view! {
        <Show when=move || !search_input.get().is_empty()>
            <div
                class="fixed inset-0 z-50 flex items-center justify-center backdrop-blur-sm"
                on:click=move |_| set_search_input.set("".to_string())
            >
                <div
                    class="bg-white rounded-lg shadow-xl max-w-md w-full mx-4 transform transition-all duration-200 ease-out"
                    on:click=prevent_close
                >
                    // Header with title and close button
                    <div class="flex items-center justify-between p-6 border-b border-gray-200">
                        <div class="gap-2 flex flex-row ">
                            <Leaf />
                            <h2 class="text-lg font-semibold text-gray-900">"Search results"</h2>
                        </div>
                        <button
                            class="text-gray-400 hover:text-gray-600 transition-colors"
                            on:click=move |_| set_search_input.set("".to_string())
                        >
                            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                                <path
                                    fill-rule="evenodd"
                                    d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
                                    clip-rule="evenodd"
                                />
                            </svg>
                        </button>
                    </div>

                    // Content area
                    <div class="p-6">
                        <SearchContent />
                    </div>
                </div>
            </div>
        </Show>
    }
}
