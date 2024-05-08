use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::route::Route;

#[component]
pub fn ClaimDone(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col grow justify-between",
            div {
                class: "flex flex-col gap-3",
                h2 {
                    "Success!"
                }
                p {
                    class: "text-lg",
                    "You have claimed your mining rewards."
                }
                p {
                    class: "text-sm text-gray-300 dark:text-gray-700",
                    "You can now spend and transfer your Mars from the dashboard."
                }
            }
            div {
                class: "flex flex-col gap-3",
                div {
                    class: "h-full"
                }
                Link {
                    class: "w-full py-3 rounded font-semibold transition-colors text-center text-white bg-orange-500 hover:bg-orange-600 active:bg-orange-700",
                    to: Route::Home{},
                    "Done"
                }
            }
        }
    }
}
