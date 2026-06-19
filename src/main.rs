use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

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

    // Target URL
    let target_url = "https://shortxlinks.in/Rs5gh46";
    println!("[+] Navigating to: {}", target_url);
    tab.navigate_to(target_url)?;
    tab.wait_until_navigated()?;

    // Step 1: "I'm Not Robot"
    // Generic XPath jo button aur link dono dhoond lega
    let robot_xpath = "//*[contains(text(), \"I'M NOT ROBOT\")]";
    if let Ok(btn) = tab.wait_for_xpath(robot_xpath) {
        println!("[+] Found: I'M NOT ROBOT. Clicking...");
        btn.click()?;
    }

    sleep(Duration::from_secs(5)).await; // Wait for load

    // Step 2: "KLIK 2X UNTUK GENERATE LINK"
    let click2x_xpath = "//*[contains(text(), \"KLIK 2X\")]";
    if let Ok(btn) = tab.wait_for_xpath(click2x_xpath) {
        println!("[+] Found: KLIK 2X. Clicking...");
        btn.click()?;
    }

    sleep(Duration::from_secs(5)).await;

    // Step 3: "LINK DOWNLOAD"
    let download_xpath = "//*[contains(text(), \"LINK DOWNLOAD\")]";
    if let Ok(btn) = tab.wait_for_xpath(download_xpath) {
        println!("[+] Found: LINK DOWNLOAD. Clicking...");
        btn.click()?;
    }

    sleep(Duration::from_secs(5)).await;

    // Step 4: Final "GET LINK"
    let get_link_xpath = "//*[contains(text(), \"GET LINK\")]";
    if let Ok(btn) = tab.wait_for_xpath(get_link_xpath) {
        println!("[+] Found: GET LINK. Finalizing...");
        btn.click()?;
        println!("[+] Task Completed!");
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("=== STARTING BOT SEQUENCE ===");
    match run_bot().await {
        Ok(_) => println!("=== SUCCESS ==="),
        Err(e) => eprintln!("=== ERROR: {} ===", e),
    }
}
