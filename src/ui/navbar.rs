use leptos::prelude::*;
use leptos_router::hooks::use_location;
use lucide_leptos::{House, Menu, Package, X};

#[component]
pub fn NavBar(title: String, children: Children) -> impl IntoView {
    let location = use_location();
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
                <h1 class="text-2xl font-bold flex-1">{title}</h1>
            </div>
            {children()}
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
                <a href="/">
                    <button class=move || {
                        if location.pathname.get() == "/".to_string() {
                            "w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors bg-green-100 text-green-800"
                        } else {
                            "w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left"
                        }
                    }>

                        <House />
                        <span>Tasks</span>
                    </button>

                </a>
                <a href="/inventory">
                    <button class=move || {
                        if location.pathname.get() == "/inventory".to_string() {
                            "w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left transition-colors bg-green-100 text-green-800"
                        } else {
                            "w-full flex items-center gap-3 px-3 py-2 rounded-lg text-left"
                        }
                    }>

                        <Package />
                        <p>Garden Inventory</p>

                    </button>
                </a>
            </nav>
        </div>
    }
}
