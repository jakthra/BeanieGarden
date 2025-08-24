use leptos::prelude::*;

#[component]
pub fn FormInputField(
    #[prop(into)] placeholder: String,
    icon: impl IntoView + 'static,
    #[prop(optional, into)] label: Option<String>,
    /// The current value of the input
    #[prop(into)]
    value: ReadSignal<String>,
    /// Callback when input value changes
    on_input: WriteSignal<String>,
) -> impl IntoView {
    view! {
        <div class="w-full">
            {label
                .map(|l| {
                    view! {
                        <label class="block text-sm font-medium text-gray-700 mb-1">{l}</label>
                    }
                })}
            <div class="relative flex items-center">
                <span class="absolute left-3 text-gray-400 flex items-center justify-center">
                    {icon}
                </span>
                <input
                    type="text"
                    placeholder=placeholder
                    prop:value=value
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        on_input.set(val.to_owned());
                    }
                    class="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:ring-2 focus:ring-green-500 focus:border-green-500 outline-none"
                />
            </div>
        </div>
    }
}
