use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::components::{
    Claim, Download, ExportKey, Home, ImportKey, Landing, Miner, Navbar, MarsTokenomics,
    PageNotFound, Send, Settings, SimpleNavbar, Stats, Tx, User, WhatIsMining,
};

#[rustfmt::skip]
#[derive(Routable, PartialEq, Clone)]
pub enum Route {
    #[route("/")]
    Landing {},

    #[layout(SimpleNavbar)]
        #[route("/what-is-mining")]
        WhatIsMining {},
        #[route("/mars-tokenomics")]
        MarsTokenomics {},
        #[route("/download")]
        Download {},
    #[end_layout]

    #[layout(Navbar)]
        #[layout(Miner)]
            #[route("/home")]
            Home {},
            #[route("/claim")]
            Claim {},
            #[route("/stats")]
            Stats {},
            #[route("/settings")]
            Settings {},
            #[route("/settings/export-key")]
            ExportKey {},
            #[route("/settings/import-key")]
            ImportKey {},
            #[route("/send/:to")]
            Send {
                to: String
            },
            #[route("/tx/:sig")]
            Tx {
                sig: String,
            },
            #[route("/u/:id")]
            User {
                id: String,
            },
        #[end_layout]
    #[end_layout]

    #[route("/:.._route")]
    PageNotFound { _route: Vec<String> }
}
