use scopeguard::defer;

async fn main_task(driver: &thirtyfour::WebDriver) -> anyhow::Result<()> {
    driver.goto("https://google.com").await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create Chrome capabilities
    let port = 9515;
    let mut caps = thirtyfour::DesiredCapabilities::chrome();

    // Launch chromedriver on port 9515
    let mut chromedriver = thirtyfour_chromedriver::manager::Handler::new()
        .launch_chromedriver(&mut caps, &port.to_string())
        .await?;

    defer!(
        let _ = chromedriver.kill();
    );

    // Connect to chrome on the same port
    let driver = thirtyfour::WebDriver::new(&format!("http://localhost:{}", port), caps).await?;

    let status = match main_task(&driver).await {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    };

    driver.quit().await?;
    status
}
