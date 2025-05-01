use crate::store::Store;
use crate::types::account::{Account, NewAccount};
use argon2::Config;
use rand::Rng;
use uuid::Uuid;
use warp::http::StatusCode;

pub fn hash_password(password: &[u8]) -> String {
    let salt = rand::rng().random::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn register(store: Store, account: NewAccount) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());

    let uuid = Uuid::new_v4().to_string();

    let account = Account {
        id: uuid,
        email: account.email,
        password: hashed_password,
        phone_number: account.phone_number,
        role: account.role,
    };

    match store.add_account(account).await {
        Ok(_) => Ok(warp::reply::with_status("Account added", StatusCode::OK)),
        Err(e) => Err(warp::reject::custom(e)),
    }
}
