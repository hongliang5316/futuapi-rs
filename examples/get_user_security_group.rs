use futuapi_rs::{
    action::user_security_group::get::GetUserSecurityGroupRequest, client,
    Qot_GetUserSecurityGroup::GroupType, Result,
};

#[tokio::main]
pub async fn main() -> Result<()> {
    let mut qot_client = client::qot_connect("127.0.0.1:11111").await?;
    let response = qot_client
        .get_user_security_group(GetUserSecurityGroupRequest::new(GroupType::GroupType_All))
        .await?;

    for group in response.groups() {
        println!("{} ({:?})", group.group_name, group.group_type);
    }

    Ok(())
}
