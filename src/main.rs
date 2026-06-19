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
    if wait_before > 0 {
        println!("[Wait] Pausing for {} seconds before searching for '{}'...", wait_before, action_name);
        sleep(Duration::from_secs(wait_before)).await;
    }

    println!("\n[Info] Current URL: {}", tab.get_url());
    println!("[Step] JS Engine searching for: '{}'...", action_name);

    // Advanced JS jo deep elements dhoondega aur hidden/timers wale buttons bypass karega
    let js_script = format!(
        r#"
        (() => {{
            const target = "{}".toLowerCase();
            const allElements = document.querySelectorAll('*');
            let bestElement = null;

            for (let el of allElements) {{
                // Bekar ke tags ko skip karo
                if (['SCRIPT', 'STYLE', 'NOSCRIPT', 'HTML', 'BODY', 'HEAD', 'META', 'IFRAME'].includes(el.tagName)) continue;

                // V.IMP: Check karo ki element actually screen par visible hai (Hidden by timer bypass)
                const style = window.getComputedStyle(el);
                if (style.display === 'none' || style.visibility === 'hidden' || el.offsetWidth === 0 || el.offsetHeight === 0) {{
                    continue;
                }}

                // Standard text ya input button ki value dono read karo
                let text = (el.innerText || el.value || "").toLowerCase().replace(/\s+/g, ' ').trim();
                
                if (text.includes(target)) {{
                    // Deepest child override karta jayega
                    bestElement = el;
                }}
            }}

            if (bestElement) {{
                // Element ko focus mein laao
                bestElement.scrollIntoView({{ behavior: 'instant', block: 'center', inline: 'center' }});
                
                // Agar nayi tab me khulne wala hai, toh usko usi page me force kholne ke liye target hatao
                if (bestElement.hasAttribute('target')) {{
                    bestElement.removeAttribute('target');
                }}
                
                // Human-like clicks bhej kar trigger karo
                ['mouseover', 'mousedown', 'mouseup', 'click'].forEach(eventType => {{
                    bestElement.dispatchEvent(new MouseEvent(eventType, {{ bubbles: true, cancelable: true, view: window }}));
                }});
                bestElement.click();
                
                return true;
            }}
            return false;
        }})()
        "#,
        keyword
    );

    let mut clicked = false;
    // Timeout bada kar 50 Seconds kar diya hai (20 attempts * 2.5s)
    // Ye har website ka timer tod dega
    for attempt in 1..=20 {
        if let Ok(remote_obj) = tab.evaluate(&js_script, true) {
            if let Some(b) = remote_obj.value.and_then(|v| v.as_bool()) {
                if b {
                    println!("[Success] ✅ JS executed and successfully clicked '{}' (Attempt {})!", action_name, attempt);
                    clicked = true;
                    break;
                }
            }
        }
        println!("[Wait] ⏳ Button hidden or timer running. Scrolling and retrying ({}/20)...", attempt);
        let _ = tab.evaluate("window.scrollBy(0, 400);", false);
        sleep(Duration::from_millis(2500)).await;
    }

    if !clicked {
        return Err(format!("Fatal: JS Engine could not locate or click '{}' after 50 seconds", action_name).into());
    }

    // Page load hone ka buffer time
    sleep(Duration::from_secs(3)).await; 
    
    // Faltu ki ad tabs block karne ka logic
    if let Ok(all_tabs) = browser.get_tabs().lock() {
        if all_tabs.len() > 1 {
            let mut tabs_to_close = Vec::new();
            for t in all_tabs.iter() {
                let url = t.get_url();
                if !url.contains("shortxlinks") && !url.contains("himalaycollege") && !url.contains("mic1") && !url.contains("about:blank") {
                    tabs_to_close.push(t.clone());
                }
            }
            
            for t in tabs_to_close {
                println!("[Ad Control] 🚫 Closing fake popup tab: {}", t.get_url());
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
    click_element_by_js(&browser, &tab, "klik 2x", "KLIK 2X BUTTON", 2).await?;
    click_element_by_js(&browser, &tab, "download", "LINK DOWNLOAD", 2).await?;

    println!("\n=========================================");
    println!("          EXECUTING ROUTINE CYCLE 2/2    ");
    println!("=========================================");
    click_element_by_js(&browser, &tab, "robot", "I'M NOT ROBOT", 2).await?;
    click_element_by_js(&browser, &tab, "klik 2x", "KLIK 2X BUTTON", 2).await?;
    click_element_by_js(&browser, &tab, "download", "LINK DOWNLOAD", 2).await?;

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
    println!("=== BOT RUNNING: ADVANCED JS INJECTION & ANTI-TIMER ACTIVE ===");
    match run_bot().await {
        Ok(_) => println!("[!] Workflow finished cleanly!"),
        Err(e) => eprintln!("[!] FATAL ERROR: {}", e),
    }
}
