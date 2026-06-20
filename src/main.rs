use headless_chrome::{Browser, LaunchOptionsBuilder, Tab};
use std::ffi::OsStr;
use std::time::Duration;
use tokio::time::sleep;

// Yeh hai hamara ORIGINAL ROBUST LOGIC jo elements ko dhoondta hai
async fn click_element_by_js(
    tab: &Tab, 
    keyword: &str, 
    action_name: &str, 
    wait_before: u64
) -> Result<(), Box<dyn std::error::Error>> {
    if wait_before > 0 {
        sleep(Duration::from_secs(wait_before)).await;
    }

    println!("[Step] Looking for: '{}'...", action_name);

    let js_script = format!(
        r#"
        (() => {{
            const target = "{}".toLowerCase();
            
            function tryClick(doc) {{
                const allElements = doc.querySelectorAll('*');
                for (let el of allElements) {{
                    const style = window.getComputedStyle(el);
                    if (style.display === 'none' || style.visibility === 'hidden') continue;

                    let content = (el.innerText + " " + el.getAttribute('alt') + " " + el.getAttribute('title') + " " + el.id).toLowerCase();
                    
                    if (content.includes(target)) {{
                        el.scrollIntoView({{ block: 'center' }});
                        // Human-like clicks
                        el.dispatchEvent(new MouseEvent('mouseover', {{ bubbles: true }}));
                        el.dispatchEvent(new MouseEvent('mousedown', {{ bubbles: true, buttons: 1 }}));
                        el.dispatchEvent(new MouseEvent('mouseup', {{ bubbles: true, buttons: 1 }}));
                        el.click();
                        return true;
                    }}
                }}
                return false;
            }}

            if (tryClick(document)) return true;
            
            const iframes = document.querySelectorAll('iframe');
            for (let iframe of iframes) {{
                try {{ if (tryClick(iframe.contentDocument || iframe.contentWindow.document)) return true; }} 
                catch(e) {{ continue; }}
            }}
            return false;
        }})()
        "#,
        keyword
    );

    let mut clicked = false;
    for attempt in 1..=20 {
        if let Ok(remote_obj) = tab.evaluate(&js_script, true) {
            if let Some(b) = remote_obj.value.and_then(|v| v.as_bool()) {
                if b {
                    println!("[Success] ✅ Clicked '{}'!", action_name);
                    clicked = true;
                    break;
                }
            }
        }
        println!("[Wait] ⏳ Scanning... ({}/20)", attempt);
        sleep(Duration::from_millis(2000)).await;
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
            OsStr::new("--user-agent=Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36"),
            OsStr::new("--disable-blink-features=AutomationControlled"),
        ])
        .build()?;
    
    let browser = Browser::new(options)?;
    let tab = browser.new_tab()?;

    println!("[Start] Navigating to target...");
    tab.navigate_to("https://shortxlinks.in/Rs5gh46")?;
    tab.wait_until_navigated()?;
    println!("[Info] Current URL: {}", tab.get_url()); 

    // --- STEP 1 ---
    click_element_by_js(&tab, "robot", "I'M NOT ROBOT", 3).await?;
    
    // --- WAIT FOR REDIRECT ---
    println!("[Wait] Clicked! Waiting for redirect...");
    sleep(Duration::from_secs(8)).await; 
    tab.wait_until_navigated()?; 
    println!("[Info] New URL: {}", tab.get_url());

    // --- STEP 2 ---
    click_element_by_js(&tab, "klik 2x", "KLIK 2X", 3).await?;
    
    // --- WAIT AGAIN ---
    sleep(Duration::from_secs(8)).await;
    tab.wait_until_navigated()?;
    println!("[Info] Final URL: {}", tab.get_url());

    println!("[Finish] Routine Completed.");
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run_bot().await {
        eprintln!("Error: {}", e);
    }
}
