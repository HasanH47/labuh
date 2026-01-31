use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::fmt;

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

impl fmt::Display for TeamRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TeamRole::Owner => "OWNER",
            TeamRole::Admin => "ADMIN",
            TeamRole::Developer => "DEVELOPER",
            TeamRole::Viewer => "VIEWER",
        };
        write!(f, "{}", s)
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
    pub user_name: Option<String>,
    pub user_email: String,
    pub role: String,
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
