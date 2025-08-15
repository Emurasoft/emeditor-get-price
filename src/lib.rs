use std::collections::HashMap;
use worker::*;

/// Price information with annual and monthly amounts.
#[derive(Clone, Copy, Debug)]
pub struct Price {
    pub annual: f64,
    pub monthly: f64,
}

thread_local! {
    static PRICES: HashMap<&'static str, Price> = get_prices();
}

pub fn get_prices() -> HashMap<&'static str, Price> {
    let mut m = HashMap::new();
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
}

#[event(fetch)]
async fn fetch(
    req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();

    // Get the headers from the incoming request
    let headers = req.headers();

    // Get the value of the "CF-IPCountry" header.
    let country = headers.get("CF-IPCountry")?;

    Response::ok(country)
}