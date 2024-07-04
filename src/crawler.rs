use headless_chrome::{Browser, LaunchOptionsBuilder};
use std::{
    error::Error,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::sleep;

#[derive(Clone, Debug)]
pub struct StoredRequestModel {
    pub url: String,
    pub body: String,
    pub response_time: Option<f64>,
}

pub async fn crawl_url(
    url: &str,
    interaction_selectors: Vec<InteractionSelector>,
) -> Result<Vec<StoredRequestModel>, Box<dyn Error>> {
    let browser_settings = LaunchOptionsBuilder::default()
        .headless(false)
        .window_size(Some((1280, 1024)))
        .build()?;

    let browser = Browser::new(browser_settings)?;
    let tab = browser.new_tab()?;

    let _ = tab.enable_fetch(None, None);

    let network_requests = Arc::new(Mutex::new(Vec::<StoredRequestModel>::new()));

    let requests = network_requests.clone();

    tab.register_response_handling(
        "handler",
        Box::new(move |response, fetch_body| {
            let mut requests = requests.lock().unwrap();

            if Option::is_none(&response.response.from_disk_cache) {
                let request_body = fetch_body().unwrap();
                requests.push(StoredRequestModel {
                    url: response.response.url.clone(),
                    body: request_body.body,
                    response_time: response.response.response_time,
                })
            };
        }),
    )?;

    tab.navigate_to(url)?;
    tab.wait_until_navigated()?;

    for interaction_selector in interaction_selectors {
        if let Ok(element) = tab.wait_for_element(&interaction_selector.selector) {
            println!("Clicking element: {:?}", &interaction_selector.selector);
            element.click()?;
        }
    }

    sleep(Duration::from_millis(10000)).await;

    let requests = network_requests.lock().unwrap().clone();

    Ok(requests)
}
