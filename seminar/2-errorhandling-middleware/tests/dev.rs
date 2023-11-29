use axum::http::StatusCode;

#[tokio::test]
async fn test_middleware() -> httpc_test::Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?;
    let result = hc.do_get("/").await?;
    result.print().await?;
    assert_eq!(result.status(), StatusCode::INTERNAL_SERVER_ERROR);
    Ok(())
}
