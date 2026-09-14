#[tokio::main]
async fn main() {
    cangnu::db::init().await;
    cangnu::db::schema::ensure_tables()
        .await
        .unwrap_or_else(|e| panic!("ensure tables: {e}"));

    cangnu::db::arcs::seed_arcs()
        .await
        .unwrap_or_else(|e| panic!("seed arcs: {e}"));

    topcoat::start(cangnu::app::router()).await.unwrap();
}
