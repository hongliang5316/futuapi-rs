use futuapi_rs::action::user_security::modify::ModifyUserSecurityRequest;
use futuapi_rs::Qot_ModifyUserSecurity::ModifyUserSecurityOp;
use futuapi_rs::{client, Result};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut client = client::qot_connect("127.0.0.1:11111").await?;
    client
        .modify_user_security(ModifyUserSecurityRequest::new(
            "自选".to_string(),
            ModifyUserSecurityOp::ModifyUserSecurityOp_Add,
            vec!["US.CEI".try_into().unwrap()],
        ))
        .await?;

    Ok(())
}
