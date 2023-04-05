use crate::models::model::{Repository, Wizard};
// use async_std::sync::RwLock;
// 使用 tokio 替换 async_std
use tokio::sync::RwLock;
use std::sync::Arc;
use tide::Result;

pub async fn build_state_server() -> Result<()> {
    femme::start();
    let state = Arc::new(RwLock::new(Repository::new()));

    let mut app = tide::with_state(state);

    app.at("/create").post(create);
    app.at("/get").get(get);

    app.listen("0.0.0.0:8888").await?;

    Ok(())
}

type State = Arc<RwLock<Repository>>;

async fn create(mut req: tide::Request<State>) -> tide::Result {
    let wizard: Wizard = req.body_json().await?;
    let state = req.state();
    let mut repo = state.write().await;
    repo.wizards.insert(wizard.name.clone(), wizard);
    Ok(tide::Response::new(200))
}

async fn get(req: tide::Request<State>) -> tide::Result {
    let state = req.state();
    let repo = &state.read().await;

    let resp = tide::Response::builder(200)
        .body(tide::Body::from_json(&repo.wizards)?)
        .build();
    Ok(resp)
}
