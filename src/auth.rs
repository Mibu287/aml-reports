pub(crate) async fn get_auth_code(driver: thirtyfour::WebDriver) -> anyhow::Result<(thirtyfour::WebDriver, ())> {
    const BASE_URL: &str = "https://amlstr.sbv.gov.vn";
    const SSO_URL: &str = "https://amlsso.sbv.gov.vn";

    // Go to website
    driver.goto(BASE_URL).await?;

    // Wait until redirected to dashboard
    let dashboard_url = format!("{}/dashboard", BASE_URL);
    tokio::time::timeout(std::time::Duration::from_secs(300), async {
        loop {
            if driver.current_url().await?.as_str() == dashboard_url {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
        Ok::<(), anyhow::Error>(())
    })
    .await??;

    // Navigate to SSO URL to get cookies
    driver.goto(SSO_URL).await?;
    let cookies = tokio::time::timeout(std::time::Duration::from_secs(300), async {
        loop {
            let cookies = match driver.get_all_cookies().await {
                Ok(cookies) => cookies,
                Err(err) => {
                    return Err(err);
                }
            };

            for cookie in &cookies {
                if cookie.name == "KC_RESTART" {
                    return Ok(cookies);
                }
            }

            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    })
    .await??;

    println!("Cookies: {:?}", cookies);

    // Go back to dashboard
    driver.goto(dashboard_url).await?;

    tokio::time::sleep(std::time::Duration::from_secs(500)).await;
    Ok((driver, ()))
}
