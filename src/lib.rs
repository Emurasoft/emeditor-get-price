use worker::*;
use phf::phf_map;
use serde::Serialize;

/// Price information with annual and monthly amounts.
#[derive(Clone, Copy, Debug)]
pub struct Price {
    pub annual: f64,
    pub monthly: f64,
}

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

/// Map from Cloudflare CF-IPCountry country code to currency code.
pub static COUNTRY_TO_CURRENCY: phf::Map<&'static str, &'static str> = phf_map! {
    // USD
    "US" => "USD",

    // JPY
    "JP" => "JPY",

    // GBP
    "GB" => "GBP",

    // EUR
    "DE" => "EUR", // Germany
    "FR" => "EUR", // France
    "IT" => "EUR", // Italy
    "ES" => "EUR", // Spain
    "NL" => "EUR", // Netherlands
    "BE" => "EUR", // Belgium
    "AT" => "EUR", // Austria
    "IE" => "EUR", // Ireland
    "PT" => "EUR", // Portugal
    "FI" => "EUR", // Finland
    "GR" => "EUR", // Greece
    "SK" => "EUR", // Slovakia
    "SI" => "EUR", // Slovenia
    "EE" => "EUR", // Estonia
    "LV" => "EUR", // Latvia
    "LT" => "EUR", // Lithuania
    "LU" => "EUR", // Luxembourg
    "CY" => "EUR", // Cyprus
    "MT" => "EUR", // Malta

    // BRL
    "BR" => "BRL",

    // CNY
    "CN" => "CNY",

    // AUD
    "AU" => "AUD",

    // KRW
    "KR" => "KRW",

    // CAD
    "CA" => "CAD",

    // TWD
    "TW" => "TWD",
};

#[derive(Serialize)]
struct PriceResponse {
    currency: String,
    annual: f64,
    monthly: f64,
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

    // Read Cloudflare's CF-IPCountry header (two-letter country code like "US", "JP").
    let country = headers
        .get("CF-IPCountry")?
        .unwrap_or_default();

    // Determine the currency; default to USD if missing/unmapped.
    let currency = COUNTRY_TO_CURRENCY
        .get(country.as_str())
        .copied()
        .unwrap_or("USD");

    // Look up the price for the resolved currency; default to USD if missing.
    let price = PRICES.get(currency).unwrap_or(&PRICES["USD"]);

    // Build a serializable response and use Response::from_json to set JSON body and headers.
    let out = PriceResponse {
        currency: currency.to_string(),
        annual: price.annual,
        monthly: price.monthly,
    };

    Response::from_json(&out)
}