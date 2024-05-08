use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::{
    components::{Banner, BannerType, Footer, MarsLogoIcon, MarsWordmarkIcon, PieIcon, UserBubble},
    gateway::AsyncResult,
    hooks::{use_appearance, use_ping},
    route::Route,
};

use super::Appearance;

#[component]
pub fn Navbar(cx: Scope) -> Element {
    let ping = use_ping(cx);
    let appearance = use_appearance(cx);
    let dark = match *appearance.read() {
        Appearance::Dark => "dark",
        Appearance::Light => "",
    };
    render! {
        div {
            class: "relative min-h-screen flex flex-col text-black dark:bg-black dark:text-white {dark}",
            if let AsyncResult::Error(_) = ping {
                render! {
                    Banner {
                        text: "Error connecting to Solana...".to_string(),
                        banner_type: BannerType::Error
                    }
                }
            }
            div {
                class: "flex w-full",
                div {
                    class: "max-w-[96rem] w-full flex flex-row justify-between mx-auto px-4 sm:px-8 py-6",
                    Link {
                        to: Route::Home {},
                        class: "flex h-10",
                        MarsWordmarkIcon {
                            class: "h-3 md:h-4 my-auto"
                        }
                    }
                    div {
                        class: "flex flex-row gap-6 md:gap-8 lg:gap-10",
                        Link {
                            class: "transition-colors flex w-10 h-10 justify-center rounded-full text-gray-300 dark:text-gray-700 hover:text-black dark:hover:text-white",
                            to: Route::Stats {},
                            PieIcon {
                                class: "w-5 h-5 sm:w-6 sm:h-6 my-auto"
                            }
                        }
                        Profile {}
                    }
                }
            }
            div {
                class: "flex flex-col h-full py-4 px-4 sm:px-8 grow w-full max-w-[96rem] mx-auto",
                Outlet::<Route> {}
            }
        }
    }
}

#[component]
pub fn Profile(cx: Scope) -> Element {
    render! {
        Link {
            to: Route::Settings {},
            UserBubble {
                class: "w-10 h-10"
            }
        }
    }
}

#[component]
pub fn SimpleNavbar(cx: Scope) -> Element {
    render! {
        div {
            class: "flex flex-col min-h-screen h-full bg-white text-black",
            div {
                class: "flex flex-row justify-between px-4 sm:px-8 py-8 w-full z-50",
                Link {
                    to: Route::Landing {},
                    class: "flex flex-row h-10",
                    MarsLogoIcon {
                        class: "h-6 md:h-8"
                    }
                }
            }
            div {
                class: "py-4 px-4 sm:px-8 grow h-full w-full max-w-[96rem] mx-auto",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}
