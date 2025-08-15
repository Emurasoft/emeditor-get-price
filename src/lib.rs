use worker::*;

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
    // This header is automatically added by Cloudflare.
    // We use unwrap_or_else to provide a default value if it's somehow missing.
    let country = headers.get("CF-IPCountry")?
        .unwrap_or_else(|| "unknown".to_string());

    console_log!(
        "Received request: method={}, url={}, country={}",
        req.method().to_string(),
        req.url()?.to_string(),
        country
    );

    Response::ok(country)
}