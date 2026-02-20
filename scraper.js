const puppeteer = require('puppeteer-extra');
const StealthPlugin = require('puppeteer-extra-plugin-stealth');

// Додаємо плагін для приховування автоматизації
puppeteer.use(StealthPlugin());

(async () => {
    console.log("🚀 Запуск звичайного Puppeteer (зі Stealth-плагіном)...");
    
    const browser = await puppeteer.launch({
        headless: false, // Обов'язково false для роботи через Xvfb
        args: [
            '--no-sandbox',
            '--disable-setuid-sandbox',
            '--disable-blink-features=AutomationControlled',
            '--window-size=1920,1080'
        ]
    });

    const page = await browser.newPage();
    
    // Встановлюємо реалістичні параметри
    await page.setViewport({ width: 1920, height: 1080 });
    await page.setUserAgent('Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36');

    try {
        console.log("🌐 Перехід на voe.com.ua...");
        // Збільшуємо таймаут, бо Cloudflare може довго думати
        await page.goto('https://www.voe.com.ua/disconnection/detailed', { 
            waitUntil: 'networkidle2', 
            timeout: 60000 
        });
        
        console.log("⏳ Чекаємо 20 секунд на випадок перевірки Cloudflare...");
        await new Promise(r => setTimeout(r, 20000));
        
        console.log("📸 Робимо скріншот...");
        await page.screenshot({ path: 'voe_detailed.png', fullPage: true });
        console.log("✅ Скріншот успішно збережено!");
        
    } catch (error) {
        console.error("❌ Помилка під час виконання:", error.message);
    } finally {
        await browser.close();
    }
})();
