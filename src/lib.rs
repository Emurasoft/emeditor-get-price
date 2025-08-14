use worker::*;

#[event(fetch)]
async fn fetch(
    req: Request,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();

    console_log!(
        "Received request: method={}, url={}",
        req.method().to_string(),
        req.url()?.to_string()
    );
    Response::ok("Hello world!")
}