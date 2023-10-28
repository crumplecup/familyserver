use dioxus::prelude::*;

pub fn Footer(cx: Scope) -> Element {
    cx.render(rsx!(
        footer {
            class: "bg-blue-200 w-full h-16 p-2 box-border gap-6 flex flex-row justify-center items-center text-teal-950",
            div {
                class: "container mx-auto flex flex-wrap p-0 flex-col md:flex-row justify-between items-center",
                "This is a footer."
            }
        }
    ))
}
