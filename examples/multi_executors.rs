use glommio::LocalExecutorBuilder;

fn main() {
    println!("Hello World!");
    let server_handle = LocalExecutorBuilder::new()
        .pin_to_cpu(1)
        .name("server")
        .spawn(|| async move {
            println!("Hello from the server!");
        }).unwrap();

    server_handle.join().unwrap();
}
