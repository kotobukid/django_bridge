use axum::{
    body::Body,
    extract::OriginalUri,
    http::{Request, Response, Uri},
    response::IntoResponse,
    routing::any,
    Router,
};
use http_body_util::BodyExt;
use hyper::client::conn::http1;
use std::net::SocketAddr;
use webapp::tokiort::TokioIo;

const PROXY_HOST: &str = "http://127.0.0.1:8000";
const PROXY_HOST_PORT: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Router::new().route("/{*path}", any(proxy_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn proxy_handler(OriginalUri(uri): OriginalUri, mut req: Request<Body>) -> impl IntoResponse {
    let target_uri = format!("{}{}", PROXY_HOST, uri);

    let url = match Uri::try_from(target_uri) {
        Ok(uri) => uri,
        Err(err) => {
            eprintln!("Invalid URI: {}", err);
            return Response::builder()
                .status(500)
                .body(Body::from("Invalid URI"))
                .unwrap();
        }
    };

    *req.uri_mut() = uri.clone();

    println!("[proxy to1] {:?}", url);

    match tokio::net::TcpStream::connect(PROXY_HOST_PORT).await {
        Ok(stream) => {
            let io = TokioIo::new(stream); // `TokioIo` を使用

            let (mut sender, connection) = match http1::handshake(io).await {
                Ok(conn) => conn,
                Err(err) => {
                    eprintln!("Handshake failed: {:?}", err);
                    return Response::builder()
                        .status(500)
                        .body(Body::from("Internal Server Error"))
                        .unwrap();
                }
            };

            tokio::spawn(async move {
                if let Err(err) = connection.await {
                    eprintln!("Connection closed with error: {:?}", err);
                }
            });

            // プロキシ先リクエスト構築
            let mut proxied_req_builder = Request::builder()
                .uri(uri.path_and_query().unwrap().to_string()) // 修正済
                .method(req.method().clone())
                .header("Host", PROXY_HOST_PORT);

            // 元のヘッダーをコピーする際に、Hostヘッダーは除外
            for (key, value) in req.headers() {
                if key.as_str().to_lowercase() != "host" {
                    // Hostヘッダーはスキップ
                    proxied_req_builder = proxied_req_builder.header(key, value);
                }
            }

            let proxied_req = proxied_req_builder.body(req.into_body()).unwrap();

            // プロキシ先へリクエスト送信
            let proxied_res = match sender.send_request(proxied_req).await {
                Ok(res) => res,
                Err(err) => {
                    eprintln!("Request failed: {:?}", err);
                    return Response::builder()
                        .status(502)
                        .body(Body::from("Bad Gateway"))
                        .unwrap();
                }
            };

            // レスポンス処理
            let mut response_builder = Response::builder().status(proxied_res.status());
            for (key, value) in proxied_res.headers() {
                response_builder = response_builder.header(key, value.clone());
            }

            let mut response_bytes = Vec::new();
            let mut proxied_body = proxied_res.into_body();

            while let Some(frame) = proxied_body.frame().await {
                match frame {
                    Ok(chunk) if chunk.is_data() => {
                        if let Some(bytes) = chunk.data_ref() {
                            response_bytes.extend_from_slice(&bytes);
                        }
                    }
                    Ok(_) => {}
                    Err(err) => {
                        eprintln!("Error reading response body: {:?}", err);
                        break;
                    }
                };
            }

            response_builder.body(Body::from(response_bytes)).unwrap()
        }
        Err(err) => {
            eprintln!("Failed to connect to target: {:?}", err);
            Response::builder()
                .status(500)
                .body(Body::from("Internal Server Error"))
                .unwrap()
        }
    }
}
