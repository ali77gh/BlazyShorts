use axum::{
    http::{Request, StatusCode},
    response::Response,
    middleware::Next,
};

pub async fn logger<B>(req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    write_to_file(&req).await;
    Ok(next.run(req).await)
}

use tokio::io::AsyncWriteExt;
use tokio::fs::OpenOptions;
async fn write_to_file<B>(req: &Request<B>){

    let content = format!("{} on {}\n", req.method(), req.uri().path());
    let file = OpenOptions::new()
        .append(true)
        .open("requests.log")
        .await;
    if let Ok(mut file) = file{
        file.write_all(content.as_bytes()).await.ok();
    }
}