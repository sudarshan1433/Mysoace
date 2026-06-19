hereuse headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

// Helper: Yeh function har click ko perform karega aur check karega
fn perform_action(tab: &Tab, xpath: &str, action_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("[Step] Searching for: {}...", action_name);
    
    // Element ka wait karo (timeout 20 sec)
    let element = tab.wait_for_xpath(xpath)?;
    element.click()?;
    
    println!("[Success] Clicked: {}. Waiting...", action_name);
    sleep(Duration::from_secs(5)).await; 
    
    Ok(())
}

async fn run_bot() -> Result<(), Box<dyn std::error::Error>> {
    let options = LaunchOptionsBuilder::default()
        .args(vec![
            OsStr::new("--no-sandbox"), 
            OsStr::new("--disable-dev-shm-usage"),
            OsStr::new("--disable-blink-features=AutomationControlled")
        ])
        .build()?;
    
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    let target_url = "https://shortxlinks.in/Rs5gh46";
    println!("[Start] Navigating to: {}", target_url);
    tab.navigate_to(target_url)?;
    tab.wait_until_navigated()?;

    // --- LOOP START: Ye sequence 2 baar chalega ---
    for i in 1..=2 {
        println!("\n=== STARTING CYCLE {}/2 ===", i);
        
        // 1. I'm Not Robot
        perform_action(&tab, "//*[contains(text(), \"I'M NOT ROBOT\")]", "I'M NOT ROBOT")?;

        // 2. Klik 2X
        perform_action(&tab, "//*[contains(text(), \"KLIK 2X\")]", "KLIK 2X")?;

        // 3. Link Download
        perform_action(&tab, "//*[contains(text(), \"LINK DOWNLOAD\")]", "LINK DOWNLOAD")?;
        
        println!("=== FINISHED CYCLE {}/2 ===\n", i);
    }
    // --- LOOP END ---

    // Final Step: Get Link
    println!("[Final] Searching for final GET LINK button...");
    perform_action(&tab, "//*[contains(text(), \"GET LINK\")]", "GET LINK")?;
    
    // Redirect ka wait
    println!("[Final] Waiting for redirect...");
    sleep(Duration::from_secs(10)).await;
    
    let final_url = tab.get_url();
    println!("\n=========================================");
    println!("[RESULT] FINAL URL REACHED: {}", final_url);
    println!("=========================================\n");

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("=== BOT STARTING: LOOP MODE ACTIVATED ===");
    match run_bot().await {
        Ok(_) => println!("[!] SEQUENCE FINISHED SUCCESSFULLY!"),
        Err(e) => eprintln!("[!] FATAL ERROR: Process stopped at step: {}", e),
    }
}
