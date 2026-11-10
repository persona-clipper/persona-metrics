use sqlx::PgPool;
use core_library_db_types::store::organizations::list::OrganizationListItem;
use core_library_db_types::store::users::user::CreateOrganizationForm;

pub async fn insert_organization(
    form: &CreateOrganizationForm,
    user_id: i32,
    db: PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO organizations(unique_name, display_name, description, owner_user_id)
        VALUES($1, $2, $3, $4);
        "#,
        form.name,
        form.display_name,
        form.description,
        user_id,
    ).execute(&db).await?;

    Ok(())

}

pub async fn get_organizations_by_user_id(
    user_id: i32,
    db: PgPool,
) -> Result<Vec<OrganizationListItem>, sqlx::Error> {
    sqlx::query_as!(
        OrganizationListItem,
        r#"
        SELECT
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