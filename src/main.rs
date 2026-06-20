use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

// Yeh function element ko dhundega aur human-like click perform karega
async fn click_element_by_js(
    tab: &Tab, 
    keyword: &str, 
    action_name: &str, 
    wait_before: u64
) -> Result<(), Box<dyn std::error::Error>> {
    if wait_before > 0 {
        sleep(Duration::from_secs(wait_before)).await;
    }

    println!("[Step] Searching for '{}'...", action_name);

    let js_script = format!(
        r#"
        (() => {{
            const target = "{}".toLowerCase();
            
            // Helper to scan document/iframe
            function tryClick(doc) {{
                const allElements = doc.querySelectorAll('*');
                for (let el of allElements) {{
                    // Skip hidden elements
                    const style = window.getComputedStyle(el);
                    if (style.display === 'none' || style.visibility === 'hidden') continue;

                    let content = (el.innerText + " " + el.getAttribute('alt') + " " + el.getAttribute('src') + " " + el.id + " " + el.className).toLowerCase();
                    
                    if (content.includes(target)) {{
                        // Scroll to element
                        el.scrollIntoView({{ block: 'center' }});
                        
                        // Human-like Click Sequence
                        el.dispatchEvent(new MouseEvent('mouseover', {{ bubbles: true }}));
                        el.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true, buttons: 1 }}));
                        el.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true, buttons: 1 }}));
                        el.click();
                        return true;
                    }}
                }}
                return false;
            }}

            // 1. Search Main Page
            if (tryClick(document)) return true;

            // 2. Search Iframes
            const iframes = document.querySelectorAll('iframe');
            for (let iframe of iframes) {{
                try {{
                    if (tryClick(iframe.contentDocument || iframe.contentWindow.document)) return true;
                }} catch(e) {{ continue; }}
            }}
            return false;
        }})()
        "#,
        keyword
    );

    let mut clicked = false;
    for attempt in 1..=15 {
        if let Ok(remote_obj) = tab.evaluate(&js_script, true) {
            if let Some(b) = remote_obj.value.and_then(|v| v.as_bool()) {
                if b {
                    println!("[Success] ✅ Clicked '{}'!", action_name);
                    clicked = true;
                    break;
                }
            }
        }
        println!("[Wait] ⏳ Scanning for '{}' ({}/15)...", action_name, attempt);
        // Force top of page periodically in case buttons are in fixed header
        let _ = tab.evaluate("window.scrollTo(0, 0);", false);
        sleep(Duration::from_millis(3000)).await;
    }

    if !clicked {
        return Err(format!("Fatal: Could not find '{}'", action_name).into());
    }
    Ok(())
}

async fn run_bot() -> Result<(), Box<dyn std::error::Error>> {
    let options = LaunchOptionsBuilder::default()
        .args(vec![
            OsStr::new("--no-sandbox"), 
            OsStr::new("--disable-dev-shm-usage"),
            OsStr::new("--window-size=1280,1024"),
            OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"),
            OsStr::new("--disable-blink-features=AutomationControlled"), // Bypass detection
            OsStr::new("--disable-infobars"),
            OsStr::new("--disable-notifications"),
        ])
        .build()?;
    
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    let target_url = "https://shortxlinks.in/Rs5gh46";
    println!("[Start] Navigating to: {}", target_url);
    tab.navigate_to(target_url)?;
    tab.wait_until_navigated()?;

    // Logic Sequence
    let sequence = vec![
        ("robot", "I'M NOT ROBOT", 3),
        ("klik 2x", "KLIK 2X", 3),
        ("download", "DOWNLOAD", 3),
        ("get link", "GET LINK", 3),
    ];

    for (keyword, name, wait) in sequence {
        click_element_by_js(&tab, keyword, name, wait).await?;
    }

    println!("[Finish] Success! URL: {}", tab.get_url());
    Ok(())
}

#[tokio::main]
async fn main() {
    match run_bot().await {
        Ok(_) => println!("Workflow finished successfully!"),
        Err(e) => eprintln!("Error: {}", e),
    }
}
