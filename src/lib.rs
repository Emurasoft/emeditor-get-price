use worker::*;
use phf::phf_map;

/// Price information with annual and monthly amounts.
#[derive(Clone, Copy, Debug)]
pub struct Price {
    pub annual: f64,
    pub monthly: f64,
}

/// Compile-time perfect hash map of currency code to price using phf.
pub static PRICES: phf::Map<&'static str, Price> = phf_map! {
    "USD" => Price { annual: 60.0, monthly: 6.0 },
    "JPY" => Price { annual: 9000.0, monthly: 900.0 },
    "GBP" => Price { annual: 45.0, monthly: 4.50 },
    "EUR" => Price { annual: 50.0, monthly: 5.0 },
    "BRL" => Price { annual: 300.0, monthly: 30.0 },
    "CNY" => Price { annual: 400.0, monthly: 40.0 },
    "AUD" => Price { annual: 90.0, monthly: 9.0 },
    "KRW" => Price { annual: 80000.0, monthly: 8000.0 },
    "CAD" => Price { annual: 80.0, monthly: 8.0 },
    "TWD" => Price { annual: 1600.0, monthly: 160.0 },
};

#[event(fetch)]
async fn fetch(
    req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();

    // Touch the PHF map to ensure it's referenced (no behavior change).
    let _ = PRICES.get("USD");

    // Get the headers from the incoming request
    let headers = req.headers();

    // Get the value of the "CF-IPCountry" header.
    let country = headers.get("CF-IPCountry")?;

    Response::ok(country.unwrap_or_default())
}