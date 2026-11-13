use sqlx::PgPool;
use core_library_db_types::store::organizations::list::OrganizationListItem;
use core_library_db_types::store::organizations::{OrganizationInfo, OrganizationMemberInfo};

pub async fn get_organizations_by_user_id(
    user_id: i32,
    db: PgPool,
) -> Result<Vec<OrganizationListItem>, sqlx::Error> {
    sqlx::query_as!(
        OrganizationListItem,
        r#"
        SELECT
            organizations.unique_hash_id,
            organizations.display_name,
            organizations.description,
            organization_roles.name AS role
        FROM
            organizations
        JOIN
            organization_roles ON organization_roles.organization_id = organizations.id
        WHERE
            organizations.owner_user_id = $1;
        "#,
        user_id
    ).fetch_all(&db).await
}

pub async fn fetch_organization_info(
    org_id: String,
    db: PgPool,
) -> Result<OrganizationInfo, sqlx::Error> {
    let common_info = sqlx::query!(
        "SELECT
            o.unique_hash_id,
            o.display_name,
            o.description,
            o.created_at,
            u.username AS owner_username
        FROM public.organizations o
            INNER JOIN public.users u
            ON u.id = o.owner_user_id
        WHERE o.unique_hash_id = $1
          AND o.deleted_at IS NULL;
        ",
        org_id
    ).fetch_one(&db).await?;

    let member_info = sqlx::query_as!(
        OrganizationMemberInfo,
        "
            SELECT
                u.id AS user_id,
                u.display_name,
                u.username,
                u.avatar_path,
                u.is_active,
                r.id AS role_id,
                r.name AS role_name
            FROM public.organization_members om
                     INNER JOIN public.organizations o
                                ON o.id = om.organization_id
                     INNER JOIN public.users u
                                ON u.id = om.member_id
                     LEFT JOIN public.organization_roles r
                               ON r.id = om.organization_role_id
            WHERE o.unique_hash_id = $1
              AND o.deleted_at IS NULL
            ORDER BY om.id ASC;
        ",
        org_id
    ).fetch_all(&db).await?;

    Ok(OrganizationInfo {
        unique_hash_id: common_info.unique_hash_id,
        display_name: common_info.display_name,
        description: common_info.description,
        created_at: common_info.created_at,
        owner_username: common_info.owner_username,
        members: member_info,
    })
}