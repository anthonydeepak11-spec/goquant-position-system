use axum::{
    extract::{Path, State, Json},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Row};
use chrono::{Utc, DateTime};

// ------------------ REQUEST STRUCTS ------------------

#[derive(Deserialize)]
pub struct OpenRequest {
    pub user_id: String,
    pub symbol: String,
    pub side: String,
    pub size_usdt: f64,
    pub leverage: i32,
    pub entry_price: f64,
}

#[derive(Deserialize)]
pub struct CloseRequest {
    pub exit_price: f64,
}

// ------------------ DB STRUCT ------------------

#[derive(Serialize, FromRow)]
pub struct PositionRow {
    pub id: String,
    pub user_id: String,
    pub symbol: String,
    pub side: String,
    pub size_usdt: f64,
    pub leverage: i32,
    pub entry_price: f64,
    pub initial_margin: f64,
    pub created_at: DateTime<Utc>,
}

// ------------------ OPEN POSITION ------------------

pub async fn open_position(
    State(pool): State<PgPool>,
    Json(req): Json<OpenRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let initial_margin = req.size_usdt / req.leverage as f64;

    let position_id = format!("pos-{}", Utc::now().timestamp_millis());

    let q = r#"
        INSERT INTO positions 
        (id, user_id, symbol, side, size_usdt, leverage, entry_price, initial_margin)
        VALUES ($1,$2,$3,$4,$5,$6,$7,$8)
    "#;

    let res = sqlx::query(q)
        .bind(&position_id)
        .bind(&req.user_id)
        .bind(&req.symbol)
        .bind(&req.side)
        .bind(req.size_usdt)
        .bind(req.leverage)
        .bind(req.entry_price)
        .bind(initial_margin)
        .execute(&pool)
        .await;

    if let Err(e) = res {
        eprintln!("❌ DB insert error: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "db insert failed" })),
        );
    }

    (
        StatusCode::CREATED,
        Json(serde_json::json!({
            "position_id": position_id,
            "initial_margin": initial_margin
        })),
    )
}

// ------------------ GET POSITION BY ID ------------------

pub async fn get_position(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
) -> (StatusCode, Json<Option<PositionRow>>) {
    let q = r#"
        SELECT id, user_id, symbol, side, size_usdt, leverage, entry_price, 
               initial_margin, created_at
        FROM positions
        WHERE id = $1
    "#;

    match sqlx::query_as::<_, PositionRow>(q)
        .bind(&id)
        .fetch_optional(&pool)
        .await
    {
        Ok(Some(row)) => (StatusCode::OK, Json(Some(row))),
        Ok(None) => (StatusCode::NOT_FOUND, Json(None)),
        Err(e) => {
            eprintln!("❌ DB select error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}

// ------------------ LIST POSITIONS ------------------

pub async fn list_positions(
    State(pool): State<PgPool>,
) -> (StatusCode, Json<Vec<PositionRow>>) {
    let q = r#"
        SELECT id, user_id, symbol, side, size_usdt, leverage, entry_price, 
               initial_margin, created_at
        FROM positions
        ORDER BY created_at DESC
        LIMIT 100
    "#;

    match sqlx::query_as::<_, PositionRow>(q).fetch_all(&pool).await {
        Ok(rows) => (StatusCode::OK, Json(rows)),
        Err(e) => {
            eprintln!("❌ DB list error: {:?}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(vec![]))
        }
    }
}

// ------------------ CLOSE POSITION ------------------

pub async fn close_position(
    State(pool): State<PgPool>,
    Path(id): Path<String>,
    Json(req): Json<CloseRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    // Fetch the open position
    let q_fetch = r#"
        SELECT size_usdt, entry_price
        FROM positions
        WHERE id = $1 AND status = 'open'
    "#;

    let res = sqlx::query(q_fetch)
        .bind(&id)
        .fetch_one(&pool)
        .await;

    let row = match res {
        Ok(r) => r,
        Err(_) => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({ "error": "position not found or already closed" })),
            )
        }
    };

    let size_usdt: f64 = row.get("size_usdt");
    let entry_price: f64 = row.get("entry_price");

    // PnL calculation
    let pnl = (req.exit_price - entry_price) * (size_usdt / entry_price);

    // Update DB
    let q_update = r#"
        UPDATE positions
        SET exit_price = $2,
            closed_at = NOW(),
            realized_pnl = $3,
            status = 'closed'
        WHERE id = $1
    "#;

    let update_res = sqlx::query(q_update)
        .bind(&id)
        .bind(req.exit_price)
        .bind(pnl)
        .execute(&pool)
        .await;

    if let Err(e) = update_res {
        eprintln!("❌ DB update error: {:?}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "failed to close position" })),
        );
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "position_id": id,
            "exit_price": req.exit_price,
            "realized_pnl": pnl
        })),
    )
}
