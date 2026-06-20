use headless_chrome::{Browser, LaunchOptionsBuilder, Tab, protocol::cdp::Input::MouseButton};
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
    println!("[Step] Engine searching for: '{}' (Coordinate Force-Click Mode)...", action_name);

    // Advanced JS jo deepest element dhoondega, usko center me scroll karega, aur uske (X, Y) coordinates dega
    let js_script = format!(
        r#"
        (() => {{
            const target = "{}".toLowerCase();
            const allElements = document.querySelectorAll('*');
            let bestElement = null;

            for (let el of allElements) {{
                if (['SCRIPT', 'STYLE', 'NOSCRIPT', 'HTML', 'BODY', 'HEAD', 'META'].includes(el.tagName)) continue;

                const style = window.getComputedStyle(el);
                if (style.display === 'none' || style.visibility === 'hidden' || el.offsetWidth === 0 || el.offsetHeight === 0) {{
                    continue;
                }}

                let text = (el.innerText || el.value || "").toLowerCase().replace(/\s+/g, ' ').trim();
                
                if (text.includes(target)) {{
                    bestElement = el; // Deepest child will override
                }}
            }}

            if (bestElement) {{
                // Scroll element to center to ensure coordinates are within viewport
                bestElement.scrollIntoView({{ behavior: 'instant', block: 'center', inline: 'center' }});
                
                const rect = bestElement.getBoundingClientRect();
                return {{
                    x: rect.left + (rect.width / 2),
                    y: rect.top + (rect.height / 2)
                }};
            }}
            return null;
        }})()
        "#,
        keyword
    );

    let mut clicked = false;
    
    // Timeout buffer: 20 attempts * 3 seconds
    for attempt in 1..=20 {
        // 1. Try in Main Frame & All Iframes
        let frames = tab.get_frames();
        for frame in frames {
            if let Ok(remote_obj) = tab.evaluate_in_frame(frame.id, &js_script, true) {
                if let Some(val) = remote_obj.value {
                    if let Some(obj) = val.as_object() {
                        let x = obj["x"].as_f64().unwrap_or(0.0);
                        let y = obj["y"].as_f64().unwrap_or(0.0);
                        
                        if x > 0.0 && y > 0.0 {
                            println!("[Success] ✅ Found '{}' at coordinates ({:.0}, {:.0}) (Attempt {})!", action_name, x, y, attempt);
                            
                            // Native browser mouse movement and physical click to bypass invisible overlays
                            let _ = tab.move_mouse(x, y);
                            sleep(Duration::from_millis(100)).await; // Human slight pause
                            let _ = tab.click_mouse(MouseButton::Left);
                            
                            clicked = true;
                            break;
                        }
                    }
                }
            }
        }

        if clicked { break; }
        
        println!("[Wait] ⏳ Button hidden or timer running. Retrying ({}/20)...", attempt);
        let _ = tab.evaluate("window.scrollBy(0, 400);", false);
        sleep(Duration::from_millis(3000)).await;
    }

    if !clicked {
        return Err(format!("Fatal: Engine could not locate or click '{}' after multiple attempts", action_name).into());
    }

    // Page load hone ka buffer time
    sleep(Duration::from_secs(4)).await; 
    
    // Faltu ki ad tabs block karne ka logic (Aapka original logic)
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
    println!("=== BOT RUNNING: COORDINATE FORCE-CLICK & ANTI-TIMER ACTIVE ===");
    match run_bot().await {
        Ok(_) => println!("[!] Workflow finished cleanly!"),
        Err(e) => eprintln!("[!] FATAL ERROR: {}", e),
    }
}
