

#[tokio::test]
async fn simple_test() -> httpc_test::Result<()> {
	let hc = httpc_test::new_client("http://localhost:3000")?;
    let result = hc.do_get("/").await?;
    result.print().await?;
    assert!(result.status().is_success());
    Ok(())
}
