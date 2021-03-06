use futuapi_rs::action::user_security::get::GetUserSecurityRequest;
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::qot_connect("127.0.0.1:11111").await?;
    let get_user_security_resp = client
        .get_user_security(GetUserSecurityRequest::new("A".into()))
        .await?;

    let code_list: Vec<_> = get_user_security_resp
        .into_inner()
        .iter()
        .map(|x| x.basic.security.to_string())
        .collect();

    println!("{:?}", code_list);

    Ok(())
}
