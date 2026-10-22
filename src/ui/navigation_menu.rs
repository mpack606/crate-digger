use dioxus::prelude::*;

#[component]
pub fn NavigationMenu() -> Element {
    rsx! {
        nav { class: "flex shrink-0 gap-4 text-[#171817] md:w-44 md:flex-col border-r border-groove-gray p-4 bg-warm-cream",
            a { href: "/", "Library" }
            a { href: "/", "Add album" }
            a { href: "/", "Labels" }
        }
    }
}
