use scopeguard::defer;

pub(crate) async fn launch_web_automation_task<TaskFuture>(
    func: fn(thirtyfour::WebDriver) -> TaskFuture,
    port: u16,
) -> anyhow::Result<()>
where
    TaskFuture: Future<Output = anyhow::Result<thirtyfour::WebDriver>>,
{
    // Create Chrome capabilities
    let mut caps = thirtyfour::DesiredCapabilities::chrome();

    // Launch chromedriver on the specified port
    let mut chromedriver = thirtyfour_chromedriver::manager::Handler::new()
        .launch_chromedriver(&mut caps, &port.to_string())
        .await?;

    defer!(
        let _ = chromedriver.kill();
    );

    // Connect to chrome on the same port
    let driver_url = format!("http://localhost:{}", port);
    let driver = thirtyfour::WebDriver::new(&driver_url, caps).await?;

    match func(driver).await {
        Ok(driver) => {
            driver.quit().await?;
            Ok(())
        }
        Err(err) => Err(err),
    }
}
