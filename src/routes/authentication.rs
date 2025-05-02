use crate::error::Error;
use crate::store::Store;
use crate::types::account::{Account, Login, NewAccount};
use argon2::Config;
use paseto::v2::local_paseto;
use rand::Rng;
use uuid::Uuid;
use warp::http::StatusCode;

pub fn hash_password(password: &[u8]) -> String {
    let salt = rand::rng().random::<[u8; 32]>();
    let config = Config::default();
    argon2::hash_encoded(password, &salt, &config).unwrap()
}

pub async fn register(
    store: Store,
    account: NewAccount,
) -> Result<impl warp::Reply, warp::Rejection> {
    let hashed_password = hash_password(account.password.as_bytes());

    let uuid = Uuid::new_v4();

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

// login logic

fn verify_password(hash: &str, password: &[u8]) -> Result<bool, argon2::Error> {
    argon2::verify_encoded(hash, password)
}

fn issue_token(account_id: Uuid) -> String {
    let state = serde_json::to_string(&account_id).expect("Failed to serialize”) state");
    local_paseto(&state, None, "RANDOM WORDS WINTER MACINTOSH PC".as_bytes())
        .expect("Failed to create token")
}

pub async fn login(store: Store, login: Login) -> Result<impl warp::Reply, warp::Rejection> {
    match store.get_account(&login).await {
        Ok(account) => match verify_password(&account.password, login.password.as_bytes()) {
            Ok(verified) => {
                if verified {
                    Ok(warp::reply::json(&issue_token(account.id)))
                } else {
                    Err(warp::reject::custom(Error::wrong_password))
                }
            }
            Err(e) => Err(warp::reject::custom(Error::argon_library_error(e))),
        },
        Err(e) => Err(warp::reject::custom(e)),
    }
}
