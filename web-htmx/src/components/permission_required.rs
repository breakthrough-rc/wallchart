use auth_service::models::UserPermission;

use rscx::{component, html, props};

#[props]
struct PermissionRequiredProps {
    #[builder(setter(into))]
    permission: UserPermission,

    #[builder(setter(into), default)]
    no_access_view: String,

    children: String,
}

#[component]
fn PermissionRequired(props: PermissionRequiredProps) -> String {
    let user = crate::context::context()
        .expect("Unable to retrieve htmx context.")
        .current_user
        .expect("No current user");

    let has_permission = user.has_perm(props.permission).await;

    if has_permission {
        props.children
    } else {
        props.no_access_view
    }
}
