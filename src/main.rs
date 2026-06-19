use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

async fn click_element_by_js(
    browser: &Browser, 
    tab: &Tab, 
    keyword: &str, 
    action_name: &str, 
    wait_before: u64
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n[Info] Current URL: {}", tab.get_url());
    
    if wait_before > 0 {
        println!("[Wait] Pausing for {} seconds...", wait_before);
        sleep(Duration::from_secs(wait_before)).await;
    }

    println!("[Step] JS Engine searching for: '{}'...", action_name);

    let js_script = format!(
        r#"
        (() => {{
            const target = "{}".toLowerCase();
            const elements = document.querySelectorAll('button, a, div, span, p, input[type="button"]');
            for (let el of elements) {{
                let text = (el.innerText || el.textContent || "").toLowerCase().trim();
                if (text.includes(target)) {{
                    el.scrollIntoView({{ behavior: 'instant', block: 'center', inline: 'center' }});
                    el.click();
                    ['mousedown', 'mouseup', 'click'].forEach(eventType => {{
                        const ev = new MouseEvent(eventType, {{ bubbles: true, cancelable: true, view: window }});
                        el.dispatchEvent(ev);
                    }});
                    return true;
                }}
            }}
            return false;
        }})()
        "#,
        keyword
    );

    let mut clicked = false;
    for attempt in 1..=6 {
        if let Ok(remote_obj) = tab.evaluate(&js_script, true) {
            if let Some(b) = remote_obj.value.and_then(|v| v.as_bool()) {
                if b {
                    println!("[Success] JS executed and successfully clicked '{}' (Attempt {})!", action_name, attempt);
                    clicked = true;
                    break;
                }
            }
        }
        println!("[Wait] Button not ready yet, scrolling down and retrying ({}/6)...", attempt);
        let _ = tab.evaluate("window.scrollBy(0, 200);", false);
        sleep(Duration::from_secs(2)).await;
    }

    if !clicked {
        return Err(format!("Fatal: JS Engine could not locate or click '{}'", action_name).into());
    }

    sleep(Duration::from_secs(2)).await; 
    
    // --- BUG FIXED HERE: Added .lock() before checking Ok() ---
    if let Ok(all_tabs) = browser.get_tabs().lock() {
        if all_tabs.len() > 1 {
            // Rust safety: list ko pehle clone kar lo taaki loop ke dauran crash na ho
            let mut tabs_to_close = Vec::new();
            for t in all_tabs.iter() {
                let url = t.get_url();
                if !url.contains("shortxlinks") && !url.contains("himalaycollege") && !url.contains("mic1") && !url.contains("about:blank") {
                    tabs_to_close.push(t.clone());
                }
            }
            
            for t in tabs_to_close {
                println!("[Ad Control] Closing fake popup tab: {}", t.get_url());
                let _ = t.close(true);
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
            OsStr::new("--window-size=1280,1024"),
            OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36"),
            OsStr::new("--disable-blink-features=AutomationControlled")
        ])
        .build()?;
    
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    let target_url = "https://shortxlinks.in/Rs5gh46";
    println!("[Start] Navigating to target link: {}", target_url);
    tab.navigate_to(target_url)?;
    tab.wait_until_navigated()?;

    println!("\n=========================================");
    println!("          EXECUTING ROUTINE CYCLE 1/2    ");
    println!("=========================================");
    click_element_by_js(&browser, &tab, "robot", "I'M NOT ROBOT", 0).await?;
    click_element_by_js(&browser, &tab, "klik 2x", "KLIK 2X BUTTON", 0).await?;
    click_element_by_js(&browser, &tab, "download", "LINK DOWNLOAD", 0).await?;

    println!("\n=========================================");
    println!("          EXECUTING ROUTINE CYCLE 2/2    ");
    println!("=========================================");
    click_element_by_js(&browser, &tab, "robot", "I'M NOT ROBOT", 2).await?;
    click_element_by_js(&browser, &tab, "klik 2x", "KLIK 2X BUTTON", 0).await?;
    click_element_by_js(&browser, &tab, "download", "LINK DOWNLOAD", 0).await?;

    println!("\n=========================================");
    println!("          FETCHING FINAL DESTINATION     ");
    println!("=========================================");
    click_element_by_js(&browser, &tab, "get link", "FINAL GET LINK", 2).await?;
    
    sleep(Duration::from_secs(6)).await;
    
    println!("\n=========================================");
    println!("[SUCCESS] BYPASS COMPLETED!");
    println!("Final Landing URL: {}", tab.get_url());
    println!("=========================================\n");

    Ok(())
}

#[tokio::main]
async fn main() {
    println!("=== BOT RUNNING: NATIVE JAVASCRIPT INJECTION ACTIVE ===");
    match run_bot().await {
        Ok(_) => println!("[!] Workflow finished cleanly!"),
        Err(e) => eprintln!("[!] FATAL ERROR: {}", e),
    }
}
