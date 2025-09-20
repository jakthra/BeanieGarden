use leptos::prelude::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Badge(
    /// The children to display inside the badge.
    children: Children,
) -> impl IntoView {
    let base_style = "inline-flex items-center rounded-full border px-2.5 py-0.5 text-xs font-semibold transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";

    let class = tw_merge!(
        base_style,
        "border-transparent  bg-green-200 text-primary-foreground hover:bg-primary/80".to_string()
    );

    view! {
        <div class=class>
            {children()}
        </div>
    }
}
