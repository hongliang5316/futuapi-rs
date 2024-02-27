use futuapi_rs::{action::user_security::get::GetUserSecurityRequest, client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut qot_client = client::qot_connect("127.0.0.1:11111").await?;
    let get_user_security_resp = qot_client
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
