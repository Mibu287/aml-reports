mod launch;

use launch::launch_web_automation_task;

async fn main_task(driver: thirtyfour::WebDriver) -> anyhow::Result<thirtyfour::WebDriver> {
    const BASE_URL: &str = "https://amlstr.sbv.gov.vn";

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

    tokio::time::sleep(std::time::Duration::from_secs(500)).await;
    Ok(driver)
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = 9515;
    let status = launch_web_automation_task(main_task, port).await;
    status
}
