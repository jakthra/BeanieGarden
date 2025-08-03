#![allow(non_snake_case)]
use leptos::prelude::*;

use lucide_leptos::{Calendar, Droplets, Sprout};
use shared::Growth;

#[component]
pub fn Growth(growth: Growth) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg border border-gray-200 shadow-sm p-4">
            <div class="flex items-start gap-3">
                <div class="p-2 bg-green-100 rounded-lg">
                    // <Sprout class="w-5 h-5 text-green-600" />
                    <Sprout />
                </div>

                <div class="flex-1 min-w-0">
                    <div class="flex items-center gap-2 mb-1">
                        <h3 class="font-medium text-sm">
                            {growth.common_plant.common_english_name}
                        </h3>
                        <span class="inline-flex items-center px-2 py-1 rounded-full text-xs font-medium border">
                            {growth.growth_type}
                        </span>
                    </div>

                    <p class="text-xs text-gray-500 italic mb-2">
                        {growth.common_plant.gbif_genus.scientific_name}
                    </p>

                    <div class="grid grid-cols-2 gap-2 mb-3 text-xs">
                        <div class="flex items-center gap-1">
                            // <Calendar class="w-3 h-3 text-gray-400" />
                            <Calendar />
                            <span class="text-gray-600">Added:</span>
                        </div>
                        <div class="flex items-center gap-1">
                            // <Droplets class="w-3 h-3 text-blue-400" />
                            <Droplets />
                            <span class="text-gray-600">{growth.age_estimate}</span>
                        </div>
                    // <div className="flex items-center gap-1">
                    // {getSunIcon(item.sunRequirement)}
                    // <span className="text-gray-600">{item.sunRequirement}</span>
                    // </div>
                    </div>

                    <p className="text-xs text-gray-600">Notes</p>
                </div>
            </div>
        </div>
    }
}
