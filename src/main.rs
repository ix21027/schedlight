use hca::create_browser_with_config;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("Starting VOE screenshot task...");

    // 1. Ініціалізація браузера: Headless = true, Розмір = 1920x1080
    // Це критично для GitHub Actions, інакше скрипт впаде без графічного інтерфейсу
    let mut browser = create_browser_with_config(true, 1920, 1080).await?;

    // 2. Увімкнення обходу захисту (на випадок Cloudflare або WAF на voe.com.ua)
    // Це фішка вашого репозиторію
    browser.apply_bot_bypass().await?;

    // 3. Навігація
    let url = "https://www.voe.com.ua/disconnection/detailed";
    println!("Navigating to: {}", url);
    browser.navigate_to(url).await?;

    // 4. Очікування завантаження контенту (SPA сайти можуть вантажитись поступово)
    println!("Waiting for page load...");
    sleep(Duration::from_secs(10)).await;

    // 5. Знімок екрана
    let filename = "voe_detailed.png";
    browser.take_screenshot(filename).await?;
    println!("Screenshot saved to: {}", filename);

    // 6. Завершення роботи
    browser.quit().await?;
    
    Ok(())
}