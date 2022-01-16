use axum::{
    routing::get,
    Router,
    extract::Path,
    http::StatusCode,
};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct Params {
    user_id: String,
    team_id: String,
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health", get(|| async { "Health: 100% OK!" }))
        .route("/users/:user_id/team/:team_id", get(users_teams_show));

    axum::Server::bind(&"0.0.0.0:3001".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn users_teams_show(
    Path(Params { user_id, team_id }): Path<Params>,
) -> Result<String, StatusCode> {
    let msg = format!("user_id: {}, team_id: {}", user_id, team_id);
    println!("{}", msg);

    let result = json!({
        "user_id": user_id,
        "team_id": team_id
    });
    let result = format!("{}", result);
    Ok(result)
}
