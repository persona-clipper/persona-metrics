use sqlx::PgPool;
use core_library_db_types::store::organizations::list::OrganizationListItem;

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
) -> Result<OrganizationListItem, sqlx::Error> {

}