use anyhow::Context;
use scopeguard::defer;

pub async fn launch_web_automation_task<TaskFuture, R>(
    func: fn(thirtyfour::WebDriver) -> TaskFuture,
) -> anyhow::Result<R>
where
    TaskFuture: Future<Output = anyhow::Result<(thirtyfour::WebDriver, R)>>,
    R: Sized,
{
    // Create Chrome capabilities
    let mut caps = thirtyfour::DesiredCapabilities::chrome();

    // Launch chromedriver on the specified port
    let (mut chromedriver, port) = thirtyfour_chromedriver::manager::Handler::new()
        .launch_chromedriver_without_port(&mut caps)
        .await
        .with_context(|| format!("Không thể khởi động chromedriver"))?;

    defer!(
        let _ = chromedriver.kill();
    );

    // Connect to chrome on the same port
    let driver_url = format!("http://localhost:{}", port);
    let driver = thirtyfour::WebDriver::new(&driver_url, caps)
        .await
        .with_context(|| {
            format!(
                "Không thể kết nối đến trình duyệt Chrome tại {}",
                driver_url
            )
        })?;

    match func(driver).await {
        Ok((driver, result)) => {
            driver
                .quit()
                .await
                .with_context(|| format!("Không thể đóng trình duyệt Chrome tại {}", driver_url))?;
            Ok(result)
        }
        Err(err) => Err(err),
    }
}
