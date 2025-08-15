use worker::*;
use phf::phf_map;
use serde::Serialize;

/// Price information with annual and monthly amounts.
#[derive(Clone, Copy, Debug)]
pub struct Price {
    pub annual: &'static str,
    pub monthly: &'static str,
}

pub static PRICES: phf::Map<&'static str, Price> = phf_map! {
    "USD" => Price { annual: "$60", monthly: "$6" },
    "JPY" => Price { annual: "9000円", monthly: "900円" },
    "GBP" => Price { annual: "£45", monthly: "£4.50" },
    "EUR" => Price { annual: "€50", monthly: "€5" },
    "BRL" => Price { annual: "R$300", monthly: "R$30" },
    "CNY" => Price { annual: "400元", monthly: "40元" },
    "AUD" => Price { annual: "A$90", monthly: "A$9" },
    "KRW" => Price { annual: "₩80000", monthly: "₩8000" },
    "CAD" => Price { annual: "C$80", monthly: "C$8" },
    "TWD" => Price { annual: "NT$1600", monthly: "NT$160" },
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

/// Resolve currency and price for a given two-letter country code (e.g., "US", "JP").
/// Falls back to USD if the country or currency is unmapped.
fn get_currency_and_price(country: &str) -> PriceResponse {
    let currency = COUNTRY_TO_CURRENCY
        .get(country)
        .copied()
        .unwrap_or("USD");

    let price = PRICES.get(currency).unwrap_or(&PRICES["USD"]);
    
    PriceResponse {
        currency,
        annual: price.annual,
        monthly: price.monthly,
    }
}

#[derive(Serialize)]
struct PriceResponse {
    currency: &'static str,
    annual: &'static str,
    monthly: &'static str,
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

    // Determine currency and price using helper function.
    let out = get_currency_and_price(country.as_str());

    Response::from_json(&out)
}