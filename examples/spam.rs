use holochain_client::AdminWebsocket;

#[tokio::main]
async fn main() {
    let port = std::env::args().nth(1).expect("Admin port not specified");

    let mut succeeded_count = 0;
    let mut failed_count = 0;
    let mut conns = Vec::new();
    loop {
        conns.push(AdminWebsocket::connect(format!("ws://127.0.0.1:{}", port)).await.unwrap());

        if conns.len() % 1_000 == 0 {
            println!("Have {} connections. Succeeded={}, failed={}", conns.len(), succeeded_count, failed_count);
        }

        let choice: usize = rand::random::<usize>() % conns.len();
        let conn = &mut conns[choice];
        match conn.list_apps(None).await {
            Ok(_) => succeeded_count += 1,
            Err(_) => failed_count += 1,
        }
    }
}
