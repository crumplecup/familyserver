use dioxus::prelude::*;

pub fn Header(cx: Scope) -> Element {
    cx.render(rsx!(
        header {
            class: "sticky top-0 z-10 text-gray-400 bg-blue-300 body-font shadow-md",
            div {
                class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
            "This is a header."
            }
        }
    ))
}
