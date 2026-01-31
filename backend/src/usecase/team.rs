use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

use crate::domain::models::team::{Team, TeamMember, TeamRole, TeamResponse};
use crate::domain::TeamRepository;
use crate::error::{AppError, Result};

pub struct TeamUsecase {
    team_repo: Arc<dyn TeamRepository>,
}

impl TeamUsecase {
    pub fn new(team_repo: Arc<dyn TeamRepository>) -> Self {
        Self { team_repo }
    }

    pub async fn create_team(&self, name: &str, owner_id: &str) -> Result<Team> {
        let id = Uuid::new_v4().to_string();
        let now = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();

        let team = Team {
            id: id.clone(),
            name: name.to_string(),
            created_at: now.clone(),
            updated_at: now.clone(),
        };

        self.team_repo.save(&team).await?;
        self.team_repo.add_member(&id, owner_id, TeamRole::Owner).await?;

        Ok(team)
    }

    pub async fn get_user_teams(&self, user_id: &str) -> Result<Vec<TeamResponse>> {
        let teams = self.team_repo.find_by_user_id(user_id).await?;
        let mut responses = Vec::new();

        for team in teams {
            let role = self.team_repo.get_user_role(&team.id, user_id).await?
                .ok_or(AppError::NotFound("Role not found for team member".to_string()))?;

            responses.push(TeamResponse {
                team,
                role,
            });
        }

        Ok(responses)
    }

    pub async fn add_member(&self, team_id: &str, user_id: &str, role: TeamRole, actor_id: &str) -> Result<()> {
        let actor_role = self.team_repo.get_user_role(team_id, actor_id).await?
            .ok_or(AppError::Forbidden("Access denied".to_string()))?;

        if actor_role != TeamRole::Owner && actor_role != TeamRole::Admin {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }

        self.team_repo.add_member(team_id, user_id, role).await?;
        Ok(())
    }

    pub async fn remove_member(&self, team_id: &str, user_id: &str, actor_id: &str) -> Result<()> {
        let actor_role = self.team_repo.get_user_role(team_id, actor_id).await?
            .ok_or(AppError::Forbidden("Access denied".to_string()))?;

        if actor_role != TeamRole::Owner && actor_role != TeamRole::Admin {
            if user_id != actor_id {
                return Err(AppError::Forbidden("Access denied".to_string()));
            }
        }

        // Cannot remove the owner
        let target_role = self.team_repo.get_user_role(team_id, user_id).await?
            .ok_or(AppError::NotFound("Member not found".to_string()))?;

        if target_role == TeamRole::Owner {
            return Err(AppError::BadRequest("Cannot remove the owner of the team".to_string()));
        }

        self.team_repo.remove_member(team_id, user_id).await?;
        Ok(())
    }

    pub async fn verify_permission(&self, team_id: &str, user_id: &str, required_role: TeamRole) -> Result<()> {
        let role = self.team_repo.get_user_role(team_id, user_id).await?
            .ok_or(AppError::Forbidden("Access denied".to_string()))?;

        let role_priority = |r: TeamRole| match r {
            TeamRole::Owner => 4,
            TeamRole::Admin => 3,
            TeamRole::Developer => 2,
            TeamRole::Viewer => 1,
        };

        if role_priority(role) < role_priority(required_role) {
            return Err(AppError::Forbidden("Access denied".to_string()));
        }

        Ok(())
    }
}
