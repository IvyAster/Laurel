use reqwest::{Request, Response};
use reqwest_middleware::{Middleware, Next, Result};
use std::time::Instant;
use tracing::{info, debug, error};
use http::Extensions;


#[derive(Clone)]
pub struct RequestLoggingMiddleware;

#[async_trait::async_trait]
impl Middleware for RequestLoggingMiddleware {
    async fn handle(
        &self,
        req: Request,
        extensions: &mut Extensions,
        next: Next<'_>,
    ) -> Result<Response> {
        let method = req.method().clone();
        let url = req.url().clone();
        let start = Instant::now();

        info!("→ {} {}", method, url);
        debug!("Request headers: {:?}", req.headers());

        if let Some(body) = req.body() {
            debug!("Request body: {:?}", body);
        }

        let result = next.run(req, extensions).await;

        match &result {
            Ok(response) => {
                let duration = start.elapsed();
                info!(
                    "← {} {} - {} - {:?}",
                    method,
                    url,
                    response.status(),
                    duration
                );
                debug!("Response headers: {:?}", response.headers());
            }
            Err(e) => {
                error!("Request failed: {} {} - {}", method, url, e);
            }
        }

        result
    }
}