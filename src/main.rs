use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

// Helper: Action perform karne ke liye
async fn perform_action(tab: &Tab, xpath: &str, action_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Debugging ke liye current URL print karein taaki pata chale bot kis page par hai
    println!("[Info] Current URL: {}", tab.get_url());
    println!("[Step] Searching for: {}...", action_name);
    
    // Element ka wait karo aur click karo
    let element = tab.wait_for_xpath(xpath)?;
    element.click()?;
    
    println!("[Success] Clicked: {}. Waiting 5 seconds...", action_name);
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

    // Shortlink sites redirect hone mein time leti hain, isliye starting mein 5s ka wait
    println!("[Wait] Waiting 5 seconds for initial page redirects...");
    sleep(Duration::from_secs(5)).await;

    // --- LOOP START: 2 Cycles ---
    for i in 1..=2 {
        println!("\n=== STARTING CYCLE {}/2 ===", i);
        
        // Super-Robust XPaths: Yeh case-insensitive hain aur deep tags ko bhi dhoond nikalenge
        perform_action(
            &tab, 
            "//*[contains(translate(., 'abcdefghijklmnopqrstuvwxyz', 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'), \"I'M NOT ROBOT\")]", 
            "I'M NOT ROBOT"
        ).await?;

        perform_action(
            &tab, 
            "//*[contains(translate(., 'abcdefghijklmnopqrstuvwxyz', 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'), \"KLIK 2X\")]", 
            "KLIK 2X"
        ).await?;

        perform_action(
            &tab, 
            "//*[contains(translate(., 'abcdefghijklmnopqrstuvwxyz', 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'), \"LINK DOWNLOAD\")]", 
            "LINK DOWNLOAD"
        ).await?;
        
        println!("=== FINISHED CYCLE {}/2 ===\n", i);
    }
    // --- LOOP END ---

    // Final Step: Get Link
    perform_action(
        &tab, 
        "//*[contains(translate(., 'abcdefghijklmnopqrstuvwxyz', 'ABCDEFGHIJKLMNOPQRSTUVWXYZ'), \"GET LINK\")]", 
        "GET LINK"
    ).await?;
    
    println!("[Final] Waiting 10 seconds for final destination load...");
    sleep(Duration::from_secs(10)).await;
    
    println!("\n=========================================");
    println!("[RESULT] FINAL URL REACHED: {}", tab.get_url());
    println!("=========================================\n");

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("=== BOT STARTING: ROBUST MODE ACTIVATED ===");
    match run_bot().await {
        Ok(_) => println!("[!] SEQUENCE FINISHED SUCCESSFULLY!"),
        Err(e) => eprintln!("[!] FATAL ERROR: Process stopped at step: {}", e),
    }
}
