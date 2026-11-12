use sqlx::PgPool;
use core_library_db_types::store::organizations::list::OrganizationListItem;
use core_library_db_types::store::users::user::CreateOrganizationForm;
use crate::model::common::generate_random_hash;

pub async fn insert_organization(
    form: &CreateOrganizationForm,
    user_id: i32,
    db: PgPool,
) -> Result<i32, sqlx::Error> {
    let mut tx = db.begin().await?;

    let unique_hash_id = generate_random_hash(8);
    let organization_response = sqlx::query!(
        r#"
        INSERT INTO organizations(unique_hash_id, display_name, description, owner_user_id)
        VALUES($1, $2, $3, $4)
        RETURNING id, owner_user_id;
        "#,
        unique_hash_id,
        form.display_name,
        form.description,
        user_id,
    ).fetch_one(&mut *tx).await?;

    let organization_role_id = sqlx::query_scalar!(
        r#"
        INSERT INTO organization_roles(organization_id, name, description)
        VALUES($1, 'Owner', 'LOL')
        RETURNING id
        "#,
        organization_response.id,
    ).fetch_one(&mut *tx).await?;

    sqlx::query!(
        r#"
        INSERT INTO organization_members(organization_id, member_id, organization_role_id)
        VALUES ($1, $2, $3)
        "#,
        organization_response.id,
        organization_response.owner_user_id,
        organization_role_id,
    ).execute(&mut *tx).await?;

    let _ = tx.commit().await;

    Ok(organization_response.id)

}

