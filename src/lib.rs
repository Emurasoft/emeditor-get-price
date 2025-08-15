use std::collections::HashMap;
use std::sync::OnceLock;
use worker::*;

/// Price information with annual and monthly amounts.
#[derive(Clone, Copy, Debug)]
pub struct Price {
    pub annual: f64,
    pub monthly: f64,
}

/// Global map from currency code (e.g., "USD") to Price.
/// Initialized on first access and then reused.
pub static PRICES: OnceLock<HashMap<&'static str, Price>> = OnceLock::new();

/// Returns a reference to the global prices map, initializing it if needed.
pub fn prices() -> &'static HashMap<&'static str, Price> {
    PRICES.get_or_init(|| {
        let mut m = HashMap::new();
        // Annual / Monthly
        m.insert("USD", Price { annual: 60.0, monthly: 6.0 });
        m.insert("JPY", Price { annual: 9000.0, monthly: 900.0 });
        m.insert("GBP", Price { annual: 45.0, monthly: 4.50 });
        m.insert("EUR", Price { annual: 50.0, monthly: 5.0 });
        m.insert("BRL", Price { annual: 300.0, monthly: 30.0 });
        m.insert("CNY", Price { annual: 400.0, monthly: 40.0 });
        m.insert("AUD", Price { annual: 90.0, monthly: 9.0 });
        m.insert("KRW", Price { annual: 80000.0, monthly: 8000.0 });
        m.insert("CAD", Price { annual: 80.0, monthly: 8.0 });
        m.insert("TWD", Price { annual: 1600.0, monthly: 160.0 });
        m
    })
}

#[event(fetch)]
async fn fetch(
    req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();

    // Touch the prices map once to ensure it's not considered dead code.
    // This does not affect the response behavior.
    let _ = prices();

    // Get the headers from the incoming request
    let headers = req.headers();

    // Get the value of the "CF-IPCountry" header.
    // This header is automatically added by Cloudflare.
    // We use unwrap_or_else to provide a default value if it's somehow missing.
    let country = headers.get("CF-IPCountry")?
        .unwrap_or_else(|| "unknown".to_string());

    Response::ok(country)
}