use dioxus::prelude::*;
use dioxus_router::components::Link;

use crate::{
    components::MarsIcon,
    gateway::AsyncResult,
    hooks::{use_mars_balance, use_proof},
    route::Route,
};

#[component]
pub fn Balance(cx: Scope) -> Element {
    let balance = use_mars_balance(cx);
    render! {
        div {
            class: "flex flex-row w-full min-h-16 rounded justify-between",
            match balance {
                AsyncResult::Ok(b) => {
                    render! {
                        div {
                            class: "flex flex-col grow gap-2 sm:gap-4",
                            h2 {
                                class: "text-lg sm:text-xl md:text-2xl font-bold",
                                "Balance"
                            }
                            div {
                                class: "flex flex-row grow justify-between",
                                div {
                                    class: "flex flex-row my-auto gap-2.5 md:gap-4",
                                    MarsIcon {
                                        class: "my-auto w-7 h-7 sm:w-8 sm:h-8 md:w-10 md:h-10"
                                    }
                                    h2 {
                                        class: "text-3xl sm:text-4xl md:text-5xl",
                                        "{b.real_number_string_trimmed()}"
                                    }
                                }
                                SendButton {}
                            }
                            UnclaimedRewards {}
                        }
                    }
                }
                _ => {
                    render! {
                        div {
                            class: "flex flex-row grow loading rounded",
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn UnclaimedRewards(cx: Scope) -> Element {
    let proof = use_proof(cx);
    if let AsyncResult::Ok(proof) = *proof.read() {
        if proof.claimable_rewards.gt(&0) {
            let rewards =
                (proof.claimable_rewards as f64) / (10f64.powf(mars::TOKEN_DECIMALS as f64));
            render! {
                div {
                    class: "flex flex-row grow justify-between mt-4 -mr-2",
                    div {
                        class: "flex flex-col gap-2",
                        p {
                            class: "font-medium text-xs text-gray-300",
                            "Mining rewards"
                        }
                        div {
                            class: "flex flex-row gap-2",
                            MarsIcon {
                                class: "my-auto w-4 h-4"
                            }
                            p {
                                class: "font-semibold",
                                "{rewards}"
                            }
                        }
                    }
                    span {
                        class: "mt-auto",
                        ClaimButton {}
                    }
                }
            }
        } else {
            None
        }
    } else {
        None
    }
}

#[component]
pub fn SendButton(cx: Scope, to: Option<String>) -> Element {
    render! {
        Link {
            to: Route::Send { to: to.clone().unwrap_or("".to_string()) },
            class: "flex h-10 w-10 my-auto rounded-full justify-center text-2xl font-bold transition-all bg-black text-white hover:shadow hover:scale-110 dark:bg-white dark:text-black",
            span {
                class: "my-auto bg-transparent",
                "↑"
            }
        }
    }
}

#[component]
pub fn ClaimButton(cx: Scope) -> Element {
    render! {
        Link {
            class: "flex transition transition-colors font-semibold px-3 h-10 rounded-full hover-100 active-200",
            to: Route::Claim {},
            span {
                class: "my-auto",
                "Claim"
            }
        }
    }
}
