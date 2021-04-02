use std::net::SocketAddr;
use warp::Filter;
use std::net::IpAddr;

#[tokio::main]
async fn main() {
    let x_forwarded_for_header =
        warp::header::optional::<String>("X-Forwarded-For").map(|addr: Option<String>| {
            if let Some(addr) = addr {
                addr.parse::<IpAddr>().ok()
            } else {
                None
            }
        });
    let remote_addr = warp::addr::remote();

    let root = warp::path::end()
        .and(x_forwarded_for_header)
        .and(remote_addr)
        .map(|addr1: Option<IpAddr>, addr2: Option<SocketAddr>| {
            if let Some(addr) = addr1 {
                format!("{}", addr)
            } else if let Some(addr) = addr2 {
                format!("{}", addr.ip())
            } else {
                "unknown error".to_string()
            }
        });

    warp::serve(root).run(([0, 0, 0, 0], 3030)).await;
}
