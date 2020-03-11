use std::env;

mod github;
mod telegram;

#[async_std::main]
pub async fn main() -> anyhow::Result<()> {
    env_logger::init();
    // server init
    let port = env::var("PORT").unwrap_or_else(|_| String::from("3000"));
    let bind_address = ["0.0.0.0:", &port].concat();
    log::info!("Server listen on {}", bind_address);
    let mut app = tide::new();
    app.at("/github_hook").post(|mut req: tide::Request<()>| async move {
        let push: github::Push = req.body_json().await.unwrap();
        log::trace!("Recv push: {:?}", &push);
        let mut msg = Vec::new();
        for commit in push.commits.into_iter() {
            msg.push(format!(
                "{} committed {} just now.\n",
                commit.author.username.unwrap_or(commit.author.name),
                commit.message
            ));
        }
        let msg = msg.concat();
        telegram::send_message(&msg).await;
        "done"
    });
    app.listen(bind_address).await?;
    Ok(())
}
