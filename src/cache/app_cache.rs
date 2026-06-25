use dashmap::DashMap;
use sqlx::PgPool;
use uuid::Uuid;

/// Cache to store roles by name and by id for searching
struct RolesCache {
    by_name: DashMap<String, Uuid>,
    by_id: DashMap<Uuid, String>,
}

impl RolesCache {
    #[tracing::instrument(
        name = "Building RolesCache",
        skip_all,
    )]
    async fn new(pool: &PgPool) -> Result<Self, Box<dyn std::error::Error>> {
        let roles = sqlx::query!(
            r#"
                SELECT id, role 
                FROM roles
             "#
        )
        .fetch_all(pool)
        .await?;

        let by_name: DashMap<String, Uuid> = DashMap::new();
        let by_id: DashMap<Uuid, String> = DashMap::new();

        for row in roles {
            by_name.insert(row.role.clone(), row.id);
            by_id.insert(row.id, row.role);
        }

        Ok(Self { by_name, by_id })
    }

    /// Returns the name of the role by the given id
    pub fn get_role_by_id(&self, id: &Uuid) -> Option<String> {
        Some(self.by_id.get(id)?.value().clone())
    }

    /// Returns the id of the role by the given name of the role
    pub fn get_role_by_name(&self, role: &str) -> Option<Uuid> {
        Some(self.by_name.get(role)?.value().clone())
    }
}

/// Struct store whole application cache
pub struct AppCache {
    pub roles: RolesCache,
}

impl AppCache {
    #[tracing::instrument(
        name = "Building Appcache",
        skip_all,
    )]
    pub async fn build(pool: &PgPool) -> Result<Self, Box<dyn std::error::Error>> {
        let roles_cache = RolesCache::new(pool).await?;

        Ok(Self { roles: roles_cache })
    }
}
