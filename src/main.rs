use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

// Smart Helper: Jo space clean karega, scroll karega aur element dhoondega
async fn perform_action(tab: &Tab, search_text: &str, action_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n[Info] Current URL: {}", tab.get_url());
    println!("[Step] Searching for Button text containing: \"{}\"...", action_name);
    
    // Ant-bot aur lazy loading trigger karne ke liye thoda scroll down code inject kiya
    let _ = tab.evaluate("window.scrollBy(0, 350);", false);
    sleep(Duration::from_secs(2)).await;

    // Robust XPath: Yeh uppercase/lowercase handles karta hai aur hidden spaces normalize karta hai
    let xpath = format!(
        "//*[contains(translate(normalize-space(.), 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'abcdefghijklmnopqrstuvwxyz'), '{}')]",
        search_text.to_lowercase()
    );

    let mut attempts = 0;
    let max_attempts = 8; // 8 * 5s = 40 seconds max wait for timers
    
    while attempts < max_attempts {
        match tab.wait_for_xpath(&xpath) {
            Ok(element) => {
                // Element ko center mein laane ke liye scroll into view
                let _ = element.scroll_into_view();
                sleep(Duration::from_secs(1)).await;
                
                element.click()?;
                println!("[Success] Clicked: {}. Waiting for page reaction...", action_name);
                sleep(Duration::from_secs(6)).await;
                return Ok(());
            }
            Err(_) => {
                attempts += 1;
                // Thoda aur scroll down agar button hidden ho
                let _ = tab.evaluate("window.scrollBy(0, 150);", false);
                println!(
                    "[Wait] '{}' not visible yet. Retrying ({}/{}) in 5s...", 
                    action_name, attempts, max_attempts
                );
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
    
    Err(format!("Timeout: Element '{}' not found on this page.", action_name).into())
}

async fn run_bot() -> Result<(), Box<dyn std::error::Error>> {
    let options = LaunchOptionsBuilder::default()
        .args(vec![
            OsStr::new("--no-sandbox"), 
            OsStr::new("--disable-dev-shm-usage"),
            OsStr::new("--window-size=1280,800"),
            OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36"),
            OsStr::new("--disable-blink-features=AutomationControlled")
        ])
        .build()?;
    
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    let target_url = "https://shortxlinks.in/Rs5gh46";
    println!("[Start] Navigating to: {}", target_url);
    tab.navigate_to(target_url)?;
    tab.wait_until_navigated()?;

    // Redirect hone ke liye initial break
    sleep(Duration::from_secs(6)).await;

    // --- LOOP START: 2 Cycles jaisa screenshots mein workflow hai ---
    for i in 1..=2 {
        println!("\n=========================================");
        println!("          STARTING ROUTINE CYCLE {}/2    ", i);
        println!("=========================================");
        
        // Step 1: I'M NOT ROBOT
        perform_action(&tab, "I'M NOT ROBOT", "I'M NOT ROBOT").await?;

        // Step 2: KLIK 2X UNTUK GENERATE LINK (Exact spelling from screen)
        perform_action(&tab, "KLIK 2X UNTUK GENERATE LINK", "KLIK 2X BUTTON").await?;

        // Step 3: LINK DOWNLOAD
        perform_action(&tab, "LINK DOWNLOAD", "LINK DOWNLOAD").await?;
    }
    // --- LOOP END ---

    // Final Step: GET LINK
    println!("\n[Final] Reached shortxlinks domain, looking for final destination...");
    perform_action(&tab, "GET LINK", "GET LINK").await?;
    
    sleep(Duration::from_secs(6)).await;
    
    println!("\n=========================================");
    println!("[SUCCESS] TARGET COMPLETED!");
    println!("Final Landing URL: {}", tab.get_url());
    println!("=========================================\n");

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("=== BOT STARTED: STRICT SPELLING & SCROLL MECHANISM ACTIVE ===");
    match run_bot().await {
        Ok(_) => println!("[!] Workflow executed cleanly!"),
        Err(e) => eprintln!("[!] FATAL ERROR: {}", e),
    }
}
