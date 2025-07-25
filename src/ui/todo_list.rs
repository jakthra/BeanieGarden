#![allow(non_snake_case)]
use crate::api::gardening_tasks::get_gardening_tasks;
use crate::entities::{GardeningTask, GardeningTaskPriority};
use crate::ui::navbar::NavBar;
use chrono::TimeDelta;
use leptos::prelude::*;
use lucide_leptos::{ChevronDown, ChevronRight, Clock, Droplets, Sprout};
use std::collections::HashSet;

#[component]
pub fn TodoList() -> impl IntoView {
    let tasks = OnceResource::new(async move { get_gardening_tasks().await });

    let (completed_tasks, set_completed_tasks) = signal(HashSet::new());
    let (expanded_tasks, set_expanded_tasks) = signal(HashSet::new());
    let tasks = vec![GardeningTask::new(
        "Prune Fruit Trees".to_string(),
        GardeningTaskPriority::High,
        TimeDelta::hours(1),
        "Remove dead, diseased, and crossing branches from dormant fruit trees.".to_string(),
        vec!["Start with dead and diseased wood removal".to_string(),
                    "Remove branches that cross or rub against each other".to_string(),
                    "Thin out crowded areas to improve air circulation".to_string(),
                    "Make clean cuts just above outward-facing buds".to_string(),
                    "Apply pruning sealant to large cuts if needed".to_string(),
        ],
        "Prune on dry days to prevent disease spread. Never remove more than 25% of the tree in one season.".to_string(),
    )];
    view! {
        <div class="max-w-md mx-auto bg-gray-50 min-h-screen relative">
            <NavBar />
            <div class="flex items-start gap-3">
                <div class="p-4 space-y-3">
                    <Suspense fallback=|| {
                        view! { <p>"Loading tasks..."</p> }
                    }>

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
                                                        GardeningTaskPriority::get_color(task.priority),
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
                    </Suspense>

                </div>
            </div>
        </div>
    }
}
