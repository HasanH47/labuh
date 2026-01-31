use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "PascalCase")]
pub enum TeamRole {
    Owner,
    Admin,
    Developer,
    Viewer,
}

impl ToString for TeamRole {
    fn to_string(&self) -> String {
        match self {
            TeamRole::Owner => "OWNER".to_string(),
            TeamRole::Admin => "ADMIN".to_string(),
            TeamRole::Developer => "DEVELOPER".to_string(),
            TeamRole::Viewer => "VIEWER".to_string(),
        }
    }
}

impl From<String> for TeamRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "OWNER" => TeamRole::Owner,
            "ADMIN" => TeamRole::Admin,
            "DEVELOPER" => TeamRole::Developer,
            "VIEWER" => TeamRole::Viewer,
            _ => TeamRole::Viewer,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct TeamMember {
    pub team_id: String,
    pub user_id: String,
    pub role: String, // Stored as String for SQLx, converted to TeamRole in logic
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct TeamResponse {
    pub team: Team,
    pub role: TeamRole,
}
