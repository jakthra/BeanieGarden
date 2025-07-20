#![allow(non_snake_case)]
use chrono::{DateTime, Local, TimeDelta};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, WildcardSegment,
};
use lucide_leptos::{ChevronDown, ChevronRight, Clock, Droplets, House, Menu, Package, Sprout, X};
use std::collections::HashSet;
use uuid::{NoContext, Timestamp, Uuid};

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/tailwind.css" />

        // sets the document title
        <Title text="Welcome to Leptos" />

        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=move || "Not found.">
                    <Route path=StaticSegment("") view=TodoList />
                    <Route path=WildcardSegment("any") view=NotFound />
                </Routes>
            </main>
        </Router>
    }
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let on_click = move |_| *count.write() += 1;

    view! {
        <h1>"Welcome to Leptos!"</h1>
        <button
            on:click=on_click
            class="bg-sky-600 hover:bg-sky-700 px-5 py-3 text-white rounded-lg"
        >
            "Click Me: "
            {count}
        </button>
    }
}

#[derive(strum_macros::Display, PartialEq)]
enum GardeningTaskPriority {
    Low,
    Medium,
    High,
}

struct GardeningTask {
    id: Uuid,
    title: String,
    priority: GardeningTaskPriority,
    time_required: TimeDelta,
    description: String,
    detailed_steps: Vec<String>,
    tips: String,
}

fn get_priority_color(value: GardeningTaskPriority) -> &'static str {
    if value == GardeningTaskPriority::High {
        return "bg-red-100 text-red-800 border-red-200";
    } else if value == GardeningTaskPriority::Medium {
        return "bg-yellow-100 text-yellow-800 border-yellow-200";
    } else if value == GardeningTaskPriority::Low {
        return "bg-green-100 text-green-800 border-green-200";
    } else {
        return "bg-gray-100 text-gray-800 border-gray-200";
    }
}

#[component]
fn TodoList() -> impl IntoView {
    let (completed_tasks, set_completed_tasks) = signal(HashSet::new());
    let (expanded_tasks, set_expanded_tasks) = signal(HashSet::new());
    let tasks = vec![GardeningTask {
        id: Uuid::new_v7(Timestamp::now(NoContext)),
        title: "Prune Fruit Trees".to_string(),
        priority: GardeningTaskPriority::High,
        description: "Remove dead, diseased, and crossing branches from dormant fruit trees.".to_string(),
        detailed_steps: vec![
                "Start with dead and diseased wood removal".to_string(),
                "Remove branches that cross or rub against each other".to_string(),
                "Thin out crowded areas to improve air circulation".to_string(),
                "Make clean cuts just above outward-facing buds".to_string(),
                "Apply pruning sealant to large cuts if needed".to_string(),
        ],
        time_required: TimeDelta::hours(1),
        tips: "Prune on dry days to prevent disease spread. Never remove more than 25% of the tree in one season.".to_string(),
    } ];
    view! {
        <div class="max-w-md mx-auto bg-gray-50 min-h-screen relative">
            <NavBar />
            <div class="flex items-start gap-3">
                <div class="p-4 space-y-3">
                    {tasks
                        .into_iter()
                        .map(|task| {
                            let id = task.id.clone();
                            let id_string = task.id.to_string();
                            view! {
                                <div class="bg-white rounded-lg border border-gray-200 shadow-sm overflow-hidden">
                                    <div class="flex items-center gap-3 p-4">
                                        <div class="relative">
                                            <input
                                                type="checkbox"
                                                id=id_string
                                                checked=move || completed_tasks.get().contains(&task.id)
                                                on:change=move |_| {
                                                    set_completed_tasks
                                                        .update(|tasks| {
                                                            if tasks.contains(&id) {
                                                                tasks.remove(&id);
                                                            } else {
                                                                tasks.insert(id);
                                                            }
                                                        });
                                                }
                                                class="w-4 h-4 text-green-600 bg-gray-100 border-gray-300 rounded focus:ring-green-500 focus:ring-2 mt-1"
                                            />
                                        </div>
                                        <div class="flex-1 min-w-0">
                                            <div class="flex items-center gap-2 mb-1">
                                                <h3 class=move || {
                                                    if completed_tasks.get().contains(&task.id) {
                                                        "font-medium text-sm line-through text-gray-500"
                                                    } else {
                                                        "font-medium text-sm"
                                                    }
                                                }>{task.title.to_string()}</h3>
                                                <span class=[
                                                    "inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border",
                                                    get_priority_color(task.priority),
                                                ]
                                                    .join(" ")>{task.priority.to_string()}</span>
                                            </div>
                                            <p class="text-xs text-gray-600 line-clamp-2">
                                                {task.description.to_string()}
                                            </p>
                                        </div>
                                        <button
                                            class="p-1 hover:bg-gray-100 rounded transition-colors"
                                            on:click=move |_| {
                                                set_expanded_tasks
                                                    .update(|tasks| {
                                                        if tasks.contains(&id) {
                                                            tasks.remove(&id);
                                                        } else {
                                                            tasks.insert(id);
                                                        }
                                                    });
                                            }
                                        >
                                            <Show
                                                when=move || { expanded_tasks.get().contains(&id) }
                                                fallback=|| view! { <ChevronRight /> }
                                            >
                                                <ChevronDown />
                                            </Show>
                                        </button>
                                    </div>
                                    <Show when=move || { expanded_tasks.get().contains(&id) }>
                                        <div class="px-4 pb-4 space-y-4">
                                            <hr class="border-gray-200" />

                                            {}
                                            <div class="grid grid-cols-2 gap-4 text-xs">
                                                <div class="flex items-center gap-2">
                                                    <Clock size=14 />
                                                    <span class="text-gray-500">Time:</span>
                                                    <span class="font-medium">
                                                        {task.time_required.num_minutes().to_string()}m
                                                    </span>
                                                </div>
                                            // <div class="flex items-center gap-2">
                                            // // {getDifficultyIcon(task.difficulty)}
                                            // <span class="text-gray-500">Difficulty:</span>
                                            // // <span class="font-medium capitalize">{task.}</span>
                                            // </div>
                                            </div>

                                            // <div class="bg-blue-50 p-3 rounded-lg">
                                            // <div class="flex items-center gap-2 mb-1">
                                            // <Sun size=14 />
                                            // <span class="text-xs font-medium text-blue-800">
                                            // Best Time
                                            // </span>
                                            // </div>
                                            // <p className="text-xs text-blue-700">{task.bestTime}</p>
                                            // </div>

                                            // <div>
                                            // <div class="flex items-center gap-2 mb-2">
                                            // <Shovel size=14 />
                                            // <span class="text-xs font-medium">Tools Needed</span>
                                            // </div>
                                            // {task.tools.map((tool, index) => (
                                            // <span
                                            // key={index}
                                            // className="inline-flex items-center px-2 py-1 rounded-md text-xs font-medium bg-gray-100 text-gray-800 border border-gray-200"
                                            // >
                                            // {tool}
                                            // </span>
                                            // ))}
                                            // <div class="flex flex-wrap gap-1"></div>
                                            // </div>

                                            {}
                                            <div class="bg-blue-50 p-3 rounded-lg">
                                                <div class="flex items-center gap-2 mb-2">
                                                    <Sprout color="#38a169" size=14 />
                                                    <span class="text-xs font-medium">Steps</span>
                                                </div>
                                                <ol class="space-y-1 text-xs text-gray-600">
                                                    {task
                                                        .detailed_steps
                                                        .clone()
                                                        .into_iter()
                                                        .enumerate()
                                                        .map(|(index, step)| {
                                                            let one_based_index = index + 1;
                                                            view! {
                                                                <li class="flex gap-2">
                                                                    <span class="text-green-600 font-medium min-w-[16px]">
                                                                        {one_based_index.to_string()}.
                                                                    </span>
                                                                    <span>{step.to_string()}</span>
                                                                </li>
                                                            }
                                                        })
                                                        .collect::<Vec<_>>()}

                                                </ol>
                                            </div>

                                            {}
                                            <div class="bg-green-50 p-3 rounded-lg">
                                                <div class="flex items-center gap-2 mb-1">
                                                    <Droplets color="#38a169" size=14 />
                                                    <span class="text-xs font-medium text-green-800">
                                                        Pro Tips
                                                    </span>
                                                </div>
                                                <p class="text-xs text-green-700">
                                                    {task.tips.to_string()}
                                                </p>
                                            </div>
                                        </div>
                                    </Show>
                                </div>
                            }
                        })
                        .collect::<Vec<_>>()}
                </div>
            </div>
        </div>
    }
}

#[component]
fn MonthCarussel() -> impl IntoView {
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

// Nav bar
#[component]
fn NavBar() -> impl IntoView {
    let (open, set_open) = signal(false);
    view! {
        <div class="sticky top-0 bg-white border-b border-gray-200 p-4 z-10">
            <div class="flex items-center gap-3 mb-4">
                <button
                    on:click=move |_| set_open.set(!open.get())
                    class="p-2 hover:bg-gray-100 rounded transition-colors"
                >
                    <Menu />
                </button>
                <h1 class="text-2xl font-bold flex-1">Tasks</h1>
            </div>
            <MonthCarussel />
        </div>
        <div class=move || {
            format!(
                "fixed left-0 top-0 h-full w-64 bg-white shadow-lg transform transition-transform duration-300 ease-in-out z-50 {}",
                if open.get() { "translate-x-0" } else { "-translate-x-full" },
            )
        }>
            <div class="p-4 border-b border-gray-200 flex items-center justify-between">
                <h2 class="font-semibold text-lg">BeanieGarden</h2>
                <button
                    class="p-1 hover:bg-gray-100 rounded transition-colors"
                    on:click=move |_| set_open.set(false)
                >
                    <X />
                </button>
            </div>
            <nav class="p-4 space-y-2">
                <button class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors bg-green-100 text-green-800">
                    <House />
                    <span>Tasks</span>
                </button>
                <button class="w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors hover:bg-gray-100">
                    <Package color="grey" />
                    <p>Garden Inventory</p>
                </button>
            </nav>
        </div>
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { <h1>"Not Found"</h1> }
}
