#[cfg(feature = "desktop")]
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use dioxus_router::prelude::*;
use ore_types::Transfer;
#[cfg(feature = "web")]
use web_time::{Duration, SystemTime, UNIX_EPOCH};

use crate::{
    components::{ActivityFilter, ActivityIndicator, Footer, MarsIcon, MarsLogoIcon},
    gateway::AsyncResult,
    hooks::{use_is_onboarded, use_mars_supply, use_transfers, use_treasury},
    utils::asset_path,
    Route,
};

#[component]
pub fn Landing(cx: Scope) -> Element {
    let is_onboarded = use_is_onboarded(cx);
    let nav = use_navigator(cx);

    // If the user is already onboarded, redirect to home.
    if is_onboarded.read().0 {
        nav.replace(Route::Home {});
    }

    render! {
        div {
            class: "flex flex-col",
            Hero{}
            Block {
                title: "Proof of work.",
                title2: "On Solana.",
                detail: "Mars uses a novel mining protocol designed for fair token distribution. It guarantees no miner can ever be starved out from earning rewards.",
                section: Section::A
            }
            Block {
                title: "Stable supply.",
                title2: "Steady growth.",
                detail: "Mars has an algorithmic supply programmed for constant linear growth. On average, ten(10) new Mars token is mined every minute by miners around the globe.",
                section: Section::B
            }
            Block {
                title: "Fair launch.",
                title2: "Immutable code.",
                detail: "Mars has no insider token allocation nor pre-mined supply. The smart contract has been frozen and open-sourced to prevent tampering or removal.",
                section: Section::C
            }
            Footer {}
        }
    }
}

#[component]
fn Navbar(cx: Scope) -> Element {
    render! {
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
    }
}

#[component]
fn Hero(cx: Scope) -> Element {
    let bg_img = asset_path("smoke.jpg");
    render! {
        div {
            class: "bg-white",
            div {
                class: "flex flex-col w-full h-screen z-20 bg-cover bg-center",
                style: "background-image: url({bg_img})",
                Navbar {}
                div {
                    class: "flex flex-col gap-y-8 sm:gap-y-10 md:gap-y-12 mx-auto my-auto pb-24 px-4 sm:px-8",
                    div {
                        class: "flex flex-col gap-y-4 sm:gap-y-6 md:gap-y-8",
                        p {
                            class: "text-center text-4xl min-[480px]:text-5xl min-[600px]:text-6xl md:text-7xl lg:text-8xl font-bold font-hero text-black",
                            // "Mars landing time!"
                            " "
                        }
                        p {
                            class: "text-xl sm:text-2xl md:text-3xl lg:text-4xl text-center max-w-[46rem] font-hero leading-7 text-black",
                            // "Mars is a cryptocurrency for sovereign individuals living in Mirascape Horizon across the galaxy."
                            " "
                            " "
                            " "
                        }
                    }
                    Link {
                        class: "mx-auto sm:text-lg md:text-xl lg:text-2xl font-semibold bg-orange-500 hover:bg-orange-600 active:bg-orange-700 text-white transition-colors rounded-full px-6 py-3",
                        to: Route::Home {},
                        "Get started →"
                    }
                }
            }
        }
    }
}

#[component]
fn Block<'a>(
    cx: Scope,
    title: &'a str,
    title2: Option<&'a str>,
    detail: &'a str,
    section: Section,
) -> Element {
    let colors = match section {
        Section::A => "bg-black text-white",
        Section::B => "bg-white text-black",
        Section::C => "bg-orange-500 text-white",
    };
    let height = match section {
        Section::A | Section::B => "min-h-svh h-full",
        Section::C => "",
    };
    render! {
        div {
            class: "flex w-full z-20 {colors} {height}",
            div {
                class: "flex flex-col h-full w-full py-16 gap-24 px-4 sm:px-8",
                div {
                    class: "flex flex-col gap-4 sm:gap-6 md:gap-8",
                    p {
                        class: "text-3xl sm:text-4xl md:text-5xl lg:text-6xl font-bold font-hero",
                        "{title}"
                        if let Some(title2) = title2 {
                            render! {
                                br{}
                                span {
                                    class: "opacity-50",
                                    "{title2}"
                                }
                            }
                        }
                    }
                    p {
                        class: "text-lg sm:text-xl md:text-2xl lg:text-3xl leading-relaxed max-w-[48rem] font-hero",
                        "{detail}"
                    }
                    BlockCta {
                        section: section
                    }
                }
                div {
                    class: "flex h-full w-full",
                    match section {
                        // MI
                        // Section::A => render! { SectionA {} },
                        Section::B => render! { SectionB {} },
                        _ => None
                    }
                }
            }
        }
    }
}

#[component]
fn BlockCta<'a>(cx: Scope, section: &'a Section) -> Element {
    match section {
        Section::A => render! {
            Link {
                class: "font-semibold mt-4",
                to: Route::WhatIsMining {},
                "Learn more →"
            }
        },
        Section::B => render! {
            Link {
                class: "font-semibold mt-4",
                to: Route::MarsTokenomics {},
                "Learn more →"
            }
        },
        Section::C => render! {
            Link {
                class: "font-semibold mt-4",
                to: "https://github.com/miraland-labs/mars",
                "Read the code →"
            }
        },
    }
}

#[derive(PartialEq, Eq)]
enum Section {
    A,
    B,
    C,
}

#[component]
fn SectionA(cx: Scope) -> Element {
    let filter = use_state(cx, || ActivityFilter::Global);
    let offset = use_state(cx, || 0);
    let (transfers, _) = use_transfers(cx, filter, offset);

    render! {
        div {
            class: "flex flex-col w-full my-auto gap-4 max-w-[48rem]",
            div {
                class: "flex flex-row gap-2",
                ActivityIndicator {}
                p {
                    class: "font-semibold text-xl opacity-50",
                    "Live transactions"
                }
            }
            div {
                class: "flex flex-col w-full",
                TransfersSection {
                    transfers: transfers
                }
            }
        }
    }
}

#[component]
fn TransfersSection(cx: Scope, transfers: AsyncResult<Vec<Transfer>>) -> Element {
    match transfers {
        AsyncResult::Ok(transfers) => {
            if transfers.is_empty() {
                render! {
                    p {
                        class: "text-sm opacity-50",
                        "No transactions yet"
                    }
                }
            } else {
                render! {
                    for (i, transfer) in transfers.iter().enumerate() {
                        if i.lt(&5) {
                            let addr = transfer.to_address[..5].to_string();
                            let amount = (transfer.amount as f64) / (10f64.powf(mars::TOKEN_DECIMALS as f64));

                            let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
                            let ts = Duration::from_secs(transfer.ts as u64);
                            let time = now.saturating_sub(ts);
                            let t = time.as_secs();
                            const ONE_MIN: u64 = 60;
                            const ONE_HOUR: u64 = ONE_MIN * 60;
                            const ONE_DAY: u64 = ONE_HOUR * 24;
                            let time_str = if t.gt(&ONE_DAY) {
                                format!("{}d ago", t.saturating_div(ONE_DAY))
                            } else if t.gt(&ONE_HOUR) {
                                format!("{}h ago", t.saturating_div(ONE_HOUR))
                            } else if t.gt(&ONE_MIN) {
                                format!("{}m ago", t.saturating_div(ONE_MIN))
                            } else {
                                format!("{}s ago", t)
                            };

                            render! {
                                div {
                                    class: "flex flex-row py-3 gap-3 w-full transition-colors rounded hover:bg-gray-900 px-2 -mx-2",
                                    div {
                                        class: "flex flex-col pt-1",
                                        p {
                                            class: "flex flex-row gap-2",
                                            span {
                                                class: "font-mono font-bold",
                                                "{addr}"
                                            }
                                            "mined "
                                            span {
                                                class: "flex flex-row font-semibold gap-0.5",
                                                MarsIcon {
                                                    class: "w-3.5 h-3.5 my-auto",
                                                }
                                                "{amount:.4}"
                                            }
                                        }
                                    }
                                    div {
                                        class: "flex pt-1.5 ml-auto",
                                        p {
                                            class: "opacity-50 text-right text-nowrap text-sm",
                                            "{time_str}"
                                        }
                                    }
                                }
                            }
                        } else {
                            None
                        }
                    }
                }
            }
        }
        _ => None,
    }
}

#[component]
fn SectionB(cx: Scope) -> Element {
    let (treasury, _) = use_treasury(cx);
    let (supply, _) = use_mars_supply(cx);
    let circulating_supply = match *treasury.read().unwrap() {
        AsyncResult::Ok(treasury) => {
            (treasury.total_claimed_rewards as f64) / 10f64.powf(mars::TOKEN_DECIMALS as f64)
        }
        _ => 0f64,
    }
    .to_string();
    let mars_supply = match supply {
        AsyncResult::Ok(token_amount) => token_amount.ui_amount.unwrap().to_string(),
        AsyncResult::Loading => "-".to_string(),
        AsyncResult::Error(_err) => "Err".to_string(),
    };
    render! {
        div {
            class: "flex flex-col gap-12 my-auto",
            MarsValue {
                title: "Circulating supply".to_string(),
                amount: circulating_supply
            }
            MarsValue {
                title: "Total supply".to_string(),
                amount: mars_supply
            }
        }
    }
}

#[component]
fn MarsValue(cx: Scope, title: String, amount: String) -> Element {
    render! {
        div {
            class: "flex flex-col gap-3",
            p {
                class: "text-gray-300 text-sm font-medium",
                "{title}"
            }
            div {
                class: "flex flex-row gap-2",
                MarsIcon {
                    class: "w-6 h-6 md:w-7 md:h-7 lg:w-8 lg:h-8 my-auto"
                }
                p {
                    class: "text-2xl md:text-3xl lg:text-4xl font-bold font-hero",
                    "{amount}"
                }
            }
        }

    }
}

#[component]
fn QuestionBreak(cx: Scope) -> Element {
    render! {
        div {
            class: "bg-orange-500 text-white w-full py-16",
            p {
                class: "text-xl sm:text-2xl md:text-3xl lg:text-4xl font-bold font-hero text-center",
                "How much will you mine?"
            }
        }
    }
}
