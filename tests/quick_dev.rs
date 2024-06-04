#![allow(unused)] // For beginning only.

use anyhow::Result;
use serde_json::json;

// cargo watch -q -c -w tests/ -x "test -q t1 -- --nocapture"
#[tokio::test]
async fn t1() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:3000")?; 

        hc.do_get("/hello?name=John").await?.print().await?;

        hc.do_get("/hello2/Mike").await?.print().await?;

        hc.do_get("/src/srcfile.rs").await?.print().await?;

        let req_login = hc.do_post(
            "/api/login",
            json!({
                "username": "demo1",
                "pwd": "welcome"
            }),
        );
        req_login.await?.print().await?;
        
        hc.do_get("/hello2/Mike").await?.print().await?;//to confirm client has received cookie after above request

        let req_create_ticket = hc.do_post(
            "/api/tickets",
            json!({
                "title": "Ticket AAA"
            }),
        );
        req_create_ticket.await?.print().await?;
        
        hc.do_delete("/api/tickets/1").await?.print().await?;

        hc.do_get("/api/tickets").await?.print().await?;
    Ok(())
}
