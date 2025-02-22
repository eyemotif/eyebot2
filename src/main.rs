mod auth;

#[tokio::main]
async fn main() {
    let access_token = tokio::fs::read_to_string("./tokens/access")
        .await
        .expect("Could not read access token")
        .trim()
        .to_owned();
    let refresh_token = tokio::fs::read_to_string("./tokens/refresh")
        .await
        .expect("Could not read refresh token")
        .trim()
        .to_owned();
    let client_id = tokio::fs::read_to_string("./tokens/clientid")
        .await
        .expect("Could not read client id")
        .trim()
        .to_owned();
    let client_secret = tokio::fs::read_to_string("./tokens/clientsecret")
        .await
        .expect("Could not read client secret")
        .trim()
        .to_owned();

    let mut auth = auth::Auth::new(refresh_token, access_token, client_id, client_secret);

    match auth.get_access_token().await {
        Ok(_) => println!("Got token!"),
        Err(err) => println!("{err:?}"),
    }
}
