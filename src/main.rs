mod internal;
#[tokio::main]
async fn main() {
    let apps = internal::methods::get_active_proccesses().await;

    for app in apps {
        println!("{}", app);
    }
}
