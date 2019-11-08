#![allow(clippy::large_enum_variant)]

#[macro_use]
extern crate seed;

use futures::Future;
use seed::prelude::*;
use seed::{fetch, Method, Request};
use serde::{Deserialize, Serialize};

const REPOSITORY_URL: &str = "https://api.github.com/repos/hew/hotfiles/branches/master";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Repo {
    pub name: String,
}

enum Loading {
    Initial,
    Loading,
    Success,
    Error
}

struct Model {
  repo: Repo,
  loading: Loading
}

impl Default for Model {
    fn default() -> Self {
        Self {
            repo: Repo {
                name: "???".into(),
            },
            loading: Loading::Initial
        }
    }
}

#[derive(Clone)]
enum Msg {
    RepoInfoFetched(fetch::ResponseDataResult<Repo>),
    FetchingRepo
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::FetchingRepo => {
            model.loading = Loading::Loading;
            orders.skip().perform_cmd(fetch_repository_info());
        },
        Msg::RepoInfoFetched(Ok(repo)) => {
            log!(format!("Response data: {:#?}", repo));
            model.repo.name = repo.name;
        },
        Msg::RepoInfoFetched(Err(_)) => { 
            log!("hey error");
            model.loading = Loading::Error;
            orders.skip();
        },
        
    }
}

fn fetch_repository_info() -> impl Future<Item = Msg, Error = Msg> {
    Request::new(REPOSITORY_URL).fetch_json_data(Msg::RepoInfoFetched)
}

fn view(model: &Model) -> Vec<Node<Msg>> {
    vec![
        md!["# Repo info"].remove(0),
        div![format!(
            "Name of the git branch we're pointed at: {}",
            model.repo.name
        )],
        raw!["<hr>"].remove(0),
        button![
            simple_ev(Ev::Click, Msg::FetchingRepo),
            "Fetch Repo"
        ],
    ]
}

// Init
fn init(_: Url, _orders: &mut impl Orders<Msg>) -> Init<Model> {
    Init::new(Model::default())
}

#[wasm_bindgen(start)]
pub fn render() {
    seed::App::build(init, update, view).build_and_start();
}
