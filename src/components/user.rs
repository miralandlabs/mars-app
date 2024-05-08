use std::str::FromStr;

use dioxus::prelude::*;
use dioxus_router::{components::Link, prelude::use_navigator};
use mars::BUS_ADDRESSES;
#[cfg(feature = "web")]
use solana_client_wasm::solana_sdk::pubkey::Pubkey;
#[cfg(feature = "desktop")]
use solana_sdk::pubkey::Pubkey;

use crate::{
    components::{
        ActivityTable, BackButton, BusBubble, Copyable, MarsIcon, SendButton, TreasuryBubble,
        UserBubble,
    },
    gateway::AsyncResult,
    hooks::{
        use_explorer_account_url, use_mars_balance_user, use_pubkey, use_user_proof,
        use_user_transfers,
    },
};

// TODO Not found

#[component]
pub fn User(cx: Scope, id: String) -> Element {
    let pubkey = use_pubkey(cx);
    let user_id = Pubkey::from_str(id);
    let nav = use_navigator(cx);

    if user_id.is_err() {
        return render! {
            p {
                "Invalid user id"
            }
        };
    }

    let user_id = user_id.unwrap();
    let balance = use_mars_balance_user(cx, user_id);
    let explorer_url = use_explorer_account_url(cx, id);
    let proof = use_user_proof(cx, user_id);
    let claimable_rewards = match proof {
        AsyncResult::Ok(proof) => {
            (proof.claimable_rewards as f64) / 10f64.powf(mars::TOKEN_DECIMALS as f64)
        }
        _ => 0.0,
    };

    let title = if let Some(index) = BUS_ADDRESSES
        .iter()
        .enumerate()
        .find(|i| (*i.1).eq(&user_id))
    {
        format!("Bus {}", index.0)
    } else if user_id.eq(&mars::TREASURY_ADDRESS) {
        "Treasury".to_string()
    } else {
        "User".to_string()
    };

    let description = if user_id.eq(&mars::TREASURY_ADDRESS) {
        Some("The treasury is a special program account responsible for minting and distributing the Mars supply.")
    } else if BUS_ADDRESSES.contains(&user_id) {
        Some("Busses are special program accounts responsible for issuing claimable rewards to miners.")
    } else {
        None
    };

    let show_send_button = title.eq("User") && user_id.ne(&pubkey);

    let container_class = "flex flex-row gap-8 justify-between py-1 sm:px-1";
    let title_class = "opacity-50 text-sm my-auto";
    let value_class = "font-medium py-1 rounded";
    let link_class = "font-medium transition-colors -ml-2 sm:ml-0 px-2 py-1 hover-100 active-200 rounded truncate";

    render! {
        div {
            class: "flex flex-col gap-16",
            div {
                class: "flex flex-col gap-3 -mt-3.5",
                BackButton {
                    onclick: move |_| {
                        nav.go_back()
                    }
                }
                div {
                    class: "flex flex-col gap-8",
                    if user_id.eq(&mars::TREASURY_ADDRESS) {
                        render! {
                            TreasuryBubble {
                                class: "my-auto w-20 h-20",
                            }
                        }
                    } else if BUS_ADDRESSES.contains(&user_id) {
                        render! {
                            BusBubble {
                                class: "my-auto w-20 h-20",
                            }
                        }
                    } else {
                        render! {
                            UserBubble {
                                class: "my-auto w-20 h-20",
                            }
                        }
                    }
                    div {
                        class: "flex flex-row justify-between",
                        h2 {
                            class: "my-auto",
                            "{title}"
                        }
                        if show_send_button {
                            render! {
                                SendButton { to: id.clone() }
                            }
                        }
                    }
                }
                if let Some(description) = description {
                    render! {
                        p {
                            class: "text-sm opacity-50 px-1",
                            "{description}"
                        }
                    }
                }
                div {
                    class: "flex flex-col gap-1",
                    div {
                        class: "{container_class} -mr-2",
                        p {
                            class: "{title_class}",
                            "ID"
                        }
                        Copyable {
                            value: id.clone(),
                            Link {
                                class: "{link_class} font-mono",
                                to: "{explorer_url}",
                                "{id}"
                            }
                        }
                    }
                    div {
                        class: "{container_class}",
                        p {
                            class: "{title_class}",
                            "Balance"
                        }
                        match balance {
                            AsyncResult::Ok(balance) => {
                                render! {
                                    span {
                                        class: "flex flex-row gap-1.5",
                                        MarsIcon {
                                            class: "w-3.5 h-3.5 my-auto",
                                        }
                                        p {
                                            class: "{value_class} truncate",
                                            "{balance.real_number_string_trimmed()}"
                                        }
                                    }
                                }
                            }
                            _ => {
                                render! {
                                    p {
                                        class: "{value_class} w-16 h-8 loading rounded",
                                    }
                                }
                            }
                        }
                    }
                    if claimable_rewards.gt(&0.0) {
                        render! {
                            div {
                                class: "{container_class}",
                                p {
                                    class: "{title_class}",
                                    "Unclaimed rewards"
                                }
                                span {
                                    class: "flex flex-row gap-1.5",
                                    MarsIcon {
                                        class: "w-3.5 h-3.5 my-auto",
                                    }
                                    p {
                                        class: "{value_class} truncate",
                                        "{claimable_rewards}"
                                   }
                                }
                            }
                        }
                    }
                }
            }
            // MI
            // UserActivity {
            //     user_id: user_id
            // }
        }
    }
}

#[component]
pub fn UserActivity(cx: Scope, user_id: Pubkey) -> Element {
    let offset = use_state(cx, || 0u64);
    let (transfers, has_more) = use_user_transfers(cx, *user_id, offset);
    match transfers {
        AsyncResult::Ok(transfers) => {
            render! {
                div {
                    class: "flex flex-col gap-4 grow w-full h-2/3 pb-20 min-h-16 rounded justify-start",
                    div {
                        class: "flex flex-row justify-between",
                        h2 {
                            class: "text-lg md:text-2xl font-bold",
                            "Activity"
                        }
                    }
                    ActivityTable{
                        offset: offset,
                        transfers: transfers,
                        has_more: has_more
                    }
                }
            }
        }
        _ => {
            render! {
                div {
                    class: "flex flex-row h-64 w-full loading rounded",
                }
            }
        }
    }
}
