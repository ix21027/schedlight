use chaser_oxide::{Browser, BrowserConfig, ChaserPage, ChaserProfile};
use chaser_oxide::page::ScreenshotParams;
use futures::StreamExt;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("🤖 Starting Chaser Oxide stealth browser...");

    // 1. Створюємо профіль, який імітує Windows, щоб обдурити Cloudflare
    let profile = ChaserProfile::windows().build();

    // 2. Налаштовуємо браузер
    // with_head() обов'язковий! Ми сховаємо вікно через Xvfb на сервері
    let config = BrowserConfig::builder()
        .with_head() 
        .window_size(1920, 1080)
        .build()
        .map_err(|e| anyhow::anyhow!("Browser config error: {:?}", e))?;

    println!("🚀 Launching browser...");
    let (mut browser, mut handler) = Browser::launch(config).await?;

    // 3. Запускаємо обробник подій браузера у фоновому потоці (обов'язково для chromiumoxide)
    let handler_task = tokio::spawn(async move {
        while let Some(_) = handler.next().await {}
    });

    // 4. Створюємо нову вкладку
    let page = browser.new_page("https://www.voe.com.ua/disconnection/detailed").await?;
    
    // 5. Обертаємо сторінку в ChaserPage для активації stealth-режиму (ізоляція змінних, патч WebGL тощо)
    let _chaser = ChaserPage::new(page.clone());

    println!("⏳ Navigating and waiting for Cloudflare challenge to pass (20s)...");
    
    // Cloudflare turnstile потребує часу, щоб виконати JS challenges та відмалювати сторінку
    sleep(Duration::from_secs(20)).await;

    println!("📸 Taking screenshot...");
    let params = ScreenshotParams::builder()
        .build()
        .map_err(|e| anyhow::anyhow!("Screenshot params error: {:?}", e))?;
        
    page.save_screenshot(params, "voe_detailed.png").await?;
    println!("✅ Success! Screenshot saved as voe_detailed.png");

    // Завершуємо роботу
    browser.close().await?;
    handler_task.abort();
    
    Ok(())
}
