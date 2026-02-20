const puppeteer = require('puppeteer');

(async () => {
    console.log("🚀 Запуск звичайного Puppeteer...");
    
    // Запускаємо стандартний headless браузер з базовими аргументами для Linux-сервера
    const browser = await puppeteer.launch({
        headless: true, 
        args: [
            '--no-sandbox',
            '--disable-setuid-sandbox',
            '--window-size=1920,1080'
        ]
    });

    const page = await browser.newPage();
    await page.setViewport({ width: 1920, height: 1080 });

    try {
        console.log("🌐 Перехід на voe.com.ua...");
        await page.goto('https://www.voe.com.ua/disconnection/detailed', { 
            waitUntil: 'networkidle2', 
            timeout: 60000 
        });
        
        console.log("⏳ Чекаємо 15 секунд для завантаження контенту...");
        await new Promise(r => setTimeout(r, 15000));
        
        console.log("📸 Робимо скріншот...");
        await page.screenshot({ path: 'voe_detailed.png', fullPage: true });
        console.log("✅ Скріншот успішно збережено!");
        
    } catch (error) {
        console.error("❌ Помилка:", error.message);
    } finally {
        await browser.close();
    }
})();
