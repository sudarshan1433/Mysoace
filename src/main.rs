use headless_chrome::{Browser, LaunchOptionsBuilder};
use rand::Rng;
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

async fn run_stealth_bot(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[+] Initializing Stealth Browser for GitHub Actions...");

    let options = LaunchOptionsBuilder::default()
        .args(vec![
            OsStr::new("--no-sandbox"),
            OsStr::new("--disable-dev-shm-usage"),
            OsStr::new("--disable-blink-features=AutomationControlled"),
        ])
        .build()?;
        
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    tab.evaluate(
        r#"
        Object.defineProperty(navigator, 'webdriver', { get: () => undefined });
        window.navigator.chrome = { runtime: {} };
        "#,
        true,
    )?;

    println!("[+] Navigating to Target: {}", url);
    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    tab.evaluate(
        r#"
        const style = document.createElement('style');
        style.innerHTML = '.ad-container, .ads, iframe { visibility: hidden !important; }';
        document.head.appendChild(style);
        "#, 
        true
    )?;

    let mut rng = rand::thread_rng();
    let delay = rng.gen_range(6..15);
    println!("[+] Waiting for {} seconds...", delay);
    sleep(Duration::from_secs(delay)).await;

    println!("[+] Searching for 'Get Link' button...");
    let xpath = "//*[contains(translate(text(), 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'abcdefghijklmnopqrstuvwxyz'), 'get link')]";
    
    if let Ok(btn) = tab.wait_for_xpath(xpath) {
        println!("[+] Button found! Clicking...");
        let _ = btn.click();
        println!("[+] Success: Click Action Completed!");
    } else {
        println!("[-] Button not found.");
    }

    sleep(Duration::from_secs(4)).await;
    Ok(())
}

#[tokio::main]
async fn main() {
    let target_url = "https://shortxlinks.in/Rs5gh46"; // Tera target URL
    
    println!("=== GITHUB ACTIONS RUN STARTED ===");
    let mut attempt = 1;
    let max_attempts = 3;

    while attempt <= max_attempts {
        println!("--- Attempt {}/{} ---", attempt, max_attempts);
        match run_stealth_bot(target_url).await {
            Ok(_) => {
                println!("[+] Task Finished Successfully.");
                break;
            },
            Err(e) => {
                eprintln!("[-] Error occurred: {}", e);
                sleep(Duration::from_secs(10)).await;
            }
        }
        attempt += 1;
    }
    println!("=== BOT SHUTDOWN ===");
}

