mod auth;
mod launch;

use auth::get_auth_code;
use launch::launch_web_automation_task;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let port = 9515;
    let _auth_code = launch_web_automation_task(get_auth_code, port).await?;
    Ok(())
}
