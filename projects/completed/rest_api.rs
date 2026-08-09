use axum::{
    Json, Router,
    extract::{Path, State},
    http::{Request, StatusCode, header},
    middleware::{self, Next},
    response::Response,
    routing::{get, post},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

const JWT_SECRET: &[u8] = b"development-only-change-me";

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
struct Item {
    id: u64,
    name: String,
    done: bool,
}

#[derive(Debug, Deserialize)]
struct CreateItem {
    name: String,
}

#[derive(Debug, Deserialize)]
struct UpdateItem {
    name: Option<String>,
    done: Option<bool>,
}

#[derive(Clone)]
struct AppState {
    items: Arc<Mutex<HashMap<u64, Item>>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[tokio::main]
async fn main() {
    let app = app();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app() -> Router {
    let state = AppState {
        items: Arc::new(Mutex::new(HashMap::new())),
    };
    let protected = Router::new()
        .route("/items", get(list_items).post(create_item))
        .route(
            "/items/{id}",
            get(get_item).put(update_item).delete(delete_item),
        )
        .route_layer(middleware::from_fn(authenticate))
        .with_state(state.clone());
    Router::new()
        .route("/login", post(login))
        .merge(protected)
        .with_state(state)
}

async fn authenticate(request: Request<axum::body::Body>, next: Next) -> Response {
    let valid = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .and_then(|token| {
            decode::<Claims>(
                token,
                &DecodingKey::from_secret(JWT_SECRET),
                &Validation::default(),
            )
            .ok()
        })
        .is_some();
    if valid {
        next.run(request).await
    } else {
        StatusCode::UNAUTHORIZED.into_response()
    }
}

async fn login() -> Result<Json<serde_json::Value>, StatusCode> {
    let claims = Claims {
        sub: "local-user".to_string(),
        exp: (chrono_like_now() + 3600) as usize,
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(serde_json::json!({ "token": token })))
}

fn chrono_like_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

async fn list_items(State(state): State<AppState>) -> Json<Vec<Item>> {
    let mut items = state
        .items
        .lock()
        .await
        .values()
        .cloned()
        .collect::<Vec<_>>();
    items.sort_by_key(|item| item.id);
    Json(items)
}

async fn create_item(
    State(state): State<AppState>,
    Json(input): Json<CreateItem>,
) -> (StatusCode, Json<Item>) {
    let mut items = state.items.lock().await;
    let id = items.keys().max().copied().unwrap_or(0) + 1;
    let item = Item {
        id,
        name: input.name,
        done: false,
    };
    items.insert(id, item.clone());
    (StatusCode::CREATED, Json(item))
}

async fn get_item(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<Json<Item>, StatusCode> {
    state
        .items
        .lock()
        .await
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_item(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<UpdateItem>,
) -> Result<Json<Item>, StatusCode> {
    let mut items = state.items.lock().await;
    let item = items.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;
    if let Some(name) = input.name {
        item.name = name;
    }
    if let Some(done) = input.done {
        item.done = done;
    }
    Ok(Json(item.clone()))
}

async fn delete_item(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> Result<StatusCode, StatusCode> {
    if state.items.lock().await.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

use axum::response::IntoResponse;

#[cfg(test)]
mod tests {
    use super::{
        AppState, CreateItem, Item, UpdateItem, create_item, delete_item, get_item, list_items,
        update_item,
    };
    use axum::{
        Json,
        extract::{Path, State},
    };
    use std::{collections::HashMap, sync::Arc};
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn list_starts_empty_and_is_sorted() {
        let state = AppState {
            items: Arc::new(Mutex::new(HashMap::from([
                (
                    2,
                    Item {
                        id: 2,
                        name: "b".into(),
                        done: false,
                    },
                ),
                (
                    1,
                    Item {
                        id: 1,
                        name: "a".into(),
                        done: true,
                    },
                ),
            ]))),
        };
        let result = list_items(axum::extract::State(state)).await;
        assert_eq!(result.0[0].id, 1);
        assert_eq!(result.0[1].id, 2);
    }

    #[tokio::test]
    async fn crud_handlers_manage_items() {
        let state = AppState {
            items: Arc::new(Mutex::new(HashMap::new())),
        };
        let (status, created) = create_item(
            State(state.clone()),
            Json(CreateItem {
                name: "learn".into(),
            }),
        )
        .await;
        assert_eq!(status, axum::http::StatusCode::CREATED);
        assert_eq!(created.0.id, 1);
        assert_eq!(
            get_item(State(state.clone()), Path(1)).await.unwrap().0,
            created.0
        );
        let updated = update_item(
            State(state.clone()),
            Path(1),
            Json(UpdateItem {
                name: Some("ship".into()),
                done: Some(true),
            }),
        )
        .await
        .unwrap();
        assert_eq!(updated.0.name, "ship");
        assert!(updated.0.done);
        assert_eq!(
            delete_item(State(state.clone()), Path(1)).await.unwrap(),
            axum::http::StatusCode::NO_CONTENT
        );
        assert!(get_item(State(state), Path(1)).await.is_err());
    }
}
