use std::{collections::HashMap, vec};

use anyhow::Context;
use duration_extender::DurationExt;

pub async fn get_auth_code(
    driver: thirtyfour::WebDriver,
) -> anyhow::Result<(thirtyfour::WebDriver, (String, String))> {
    const BASE_URL: &str = "https://amlstr.sbv.gov.vn";
    const SSO_URL: &str = "https://amlsso.sbv.gov.vn";

    // Go to website
    driver
        .goto(BASE_URL)
        .await
        .with_context(|| format!("Chrome không thể mở đường dẫn {}", BASE_URL))?;

    // Wait until redirected to dashboard
    let dashboard_url = format!("{}/dashboard", BASE_URL);
    tokio::time::timeout(std::time::Duration::from_secs(300), async {
        loop {
            if driver.current_url().await?.as_str() == dashboard_url {
                break;
            }
            tokio::time::sleep(100.milliseconds()).await;
        }
        Ok::<(), anyhow::Error>(())
    })
    .await
    .with_context(|| {
        format!("Trình duyệt Chrome không được trở lại trang chủ. Đăng nhập không thành công?")
    })??;

    // Navigate to SSO to trigger cookie creation
    driver
        .goto(SSO_URL)
        .await
        .with_context(|| format!("Trình duyệt Chrome không thể mở đường dẫn {}", SSO_URL))?;

    tokio::time::timeout(300.seconds(), async {
        loop {
            tokio::time::sleep(1.seconds()).await;

            let cookies = match driver.get_all_cookies().await {
                Ok(cookies) => cookies,
                Err(err) => {
                    return Err(err);
                }
            };

            for cookie in &cookies {
                if cookie.name == "KC_RESTART" {
                    return Ok(());
                }
            }
        }
    })
    .await
    .with_context(|| {
        format!(
            "Trình duyệt Chrome không lấy được cookie từ trang {}",
            SSO_URL
        )
    })??;

    // Get local storage items

    let (auth_key, auth_value) = tokio::time::timeout(300.seconds(), async {
        let report_url = format!("{}/report-str/report-two", BASE_URL);

        loop {
            driver.goto(&report_url).await?;
            tokio::time::sleep(1.seconds()).await;

            let script: &str = r#"
                let ls = window.localStorage, results = {}
                for (var i = 0; i < ls.length; ++i) {
                    results[ls.key(i)] = ls.getItem(ls.key(i));
                }
                return results;
            "#;

            let storage = driver.execute(script, vec![]).await?;

            if let Ok(object) = storage.convert::<HashMap<String, String>>() {
                for (key, value) in object {
                    let auth_key_pattern = regex::Regex::new(r"^v\d+\.\d+\.\d+-auth\w+$").unwrap();
                    if auth_key_pattern.is_match(&key) {
                        return anyhow::Result::<(String, String)>::Ok((key, value));
                    }
                }
            }
        }
    })
    .await
    .with_context(|| {
        format!(
            "Trình duyệt Chrome không lấy được access token từ trang {}",
            SSO_URL
        )
    })??;

    Ok((driver, (auth_key, auth_value)))
}
