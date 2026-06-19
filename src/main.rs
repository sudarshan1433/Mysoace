use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

// Helper: Isme 'wait_before' parameter add kiya hai taaki control hamare hath me ho
async fn perform_action(
    browser: &Browser, 
    tab: &Tab, 
    search_text: &str, 
    action_name: &str, 
    wait_before: u64
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n[Info] Current URL: {}", tab.get_url());
    
    // Agar wait_before 0 se bada hai, tabhi rukega, nahi toh skip!
    if wait_before > 0 {
        println!("[Wait] Letting ads load for {} seconds...", wait_before);
        sleep(Duration::from_secs(wait_before)).await;
    } else {
        println!("[Instant] No wait required for this page. Targetting immediately!");
    }

    let xpath = format!(
        "//*[contains(translate(normalize-space(.), 'ABCDEFGHIJKLMNOPQRSTUVWXYZ', 'abcdefghijklmnopqrstuvwxyz'), '{}')]",
        search_text.to_lowercase()
    );

    let element = tab.wait_for_xpath(&xpath)?;
    element.scroll_into_view()?;
    element.click()?;
    println!("[Success] Clicked: {}", action_name);

    // Click ke baad khulne wale fake ad popups ko handle karna
    sleep(Duration::from_secs(2)).await; 
    if let Ok(all_tabs) = browser.get_tabs() {
        if all_tabs.len() > 1 {
            for t in all_tabs {
                let url = t.get_url();
                if !url.contains("shortxlinks") && !url.contains("himalaycollege") && !url.contains("mic1") && !url.contains("about:blank") {
                    println!("[Ad Control] Closing fake popup tab: {}", url);
                    let _ = t.close(true);
                }
            }
        }
    }

    Ok(())
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

    // --- CYCLE 1: Jaisa tumne kaha, yahan turant bina kisi wait ke click hoga ---
    println!("\n=========================================");
    println!("          STARTING ROUTINE CYCLE 1/2     ");
    println!("=========================================");
    
    // Last parameter '0' lagaya hai taaki page load hote hi INSTANT click ho jaye
    perform_action(&browser, &tab, "I'M NOT ROBOT", "I'M NOT ROBOT (Page 1)", 0).await?;
    perform_action(&browser, &tab, "KLIK 2X UNTUK GENERATE LINK", "KLIK 2X BUTTON", 0).await?;
    perform_action(&browser, &tab, "LINK DOWNLOAD", "LINK DOWNLOAD", 0).await?;


    // --- CYCLE 2: Agar aage ke pages par ad load hone ka wait chahiye toh yahan '4' ya '0' kar sakte ho ---
    println!("\n=========================================");
    println!("          STARTING ROUTINE CYCLE 2/2     ");
    println!("=========================================");
    
    // Agar lagta hai ki second cycle mein bhi instant kaam ho raha hai, toh in '4' ko bhi '0' kar dena
    perform_action(&browser, &tab, "I'M NOT ROBOT", "I'M NOT ROBOT (Page 2)", 4).await?;
    perform_action(&browser, &tab, "KLIK 2X UNTUK GENERATE LINK", "KLIK 2X BUTTON", 4).await?;
    perform_action(&browser, &tab, "LINK DOWNLOAD", "LINK DOWNLOAD", 4).await?;


    // Final Step: GET LINK (Ispe 3 second ka safe ad wait diya hai)
    println!("\n[Final] Looking for final destination link...");
    perform_action(&browser, &tab, "GET LINK", "GET LINK", 3).await?;
    
    sleep(Duration::from_secs(5)).await;
    
    println!("\n=========================================");
    println!("[SUCCESS] TARGET COMPLETED!");
    println!("Final Landing URL: {}", tab.get_url());
    println!("=========================================\n");

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("=== BOT STARTED: MULTI-SPEED MODE ACTIVATED ===");
    match run_bot().await {
        Ok(_) => println!("[!] Workflow executed cleanly!"),
        Err(e) => eprintln!("[!] FATAL ERROR: {}", e),
    }
}
