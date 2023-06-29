use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct GitLabMergeRequestEvent {
    pub object_kind: String,
    pub event_type: String,
    pub user: User,
    pub project: Project,
    pub repository: Repository,
    pub object_attributes: ObjectAttributes,
    pub labels: Vec<Label>,
    pub changes: Changes,
    pub assignees: Option<Vec<User>>,
    pub reviewers: Option<Vec<User>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub username: String,
    pub avatar_url: String,
    pub email: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<u64>,
    pub name: String,
    pub description: String,
    pub web_url: String,
    pub avatar_url: Option<String>,
    pub git_ssh_url: String,
    pub git_http_url: String,
    pub namespace: String,
    pub visibility_level: u64,
    pub path_with_namespace: String,
    pub default_branch: String,
    pub ci_config_path: Option<String>,
    pub homepage: String,
    pub url: String,
    pub ssh_url: String,
    pub http_url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
    pub description: String,
    pub homepage: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectAttributes {
    pub id: u64,
    pub iid: u64,
    pub target_branch: String,
    pub source_branch: String,
    pub source_project_id: u64,
    pub author_id: u64,
    pub assignee_ids: Vec<u64>,
    pub assignee_id: Option<u64>,
    pub reviewer_ids: Vec<u64>,
    pub title: String,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub last_edited_at: Option<String>,
    pub last_edited_by_id: Option<u64>,
    pub milestone_id: Option<u64>,
    pub state_id: u64,
    pub state: String,
    pub blocking_discussions_resolved: bool,
    pub work_in_progress: bool,
    pub first_contribution: bool,
    pub merge_status: String,
    pub target_project_id: u64,
    pub description: String,
    pub total_time_spent: u64,
    pub time_change: u64,
    pub human_total_time_spent: Option<String>,
    pub human_time_change: Option<String>,
    pub human_time_estimate: Option<String>,
    pub updated_by_id: Option<u64>,
    pub url: String,
    pub source: Project,
    pub target: Project,
    pub last_commit: LastCommit,
    pub labels: Vec<Label>,
    pub action: Option<String>,
    pub detailed_merge_status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastCommit {
    pub id: String,
    pub message: String,
    pub title: String,
    pub timestamp: String,
    pub url: String,
    pub author: Author,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Label {
    pub id: u64,
    pub title: String,
    pub color: String,
    pub project_id: u64,
    pub created_at: String,
    pub updated_at: Option<String>,
    pub template: bool,
    pub description: String,
    pub r#type: String,
    pub group_id: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Changes {
    pub updated_by_id: Option<CurrentPrevious<u64>>,
    pub updated_at: Option<CurrentPrevious<String>>,
    pub labels: Option<CurrentPrevious<Vec<Label>>>,
    pub last_edited_at: Option<CurrentPrevious<Option<String>>>,
    pub last_edited_by_id: Option<CurrentPrevious<u64>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentPrevious<T> {
    pub previous: Option<T>,
    pub current: T,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize() {
        let json = include_str!("../data/event.json");
        let event: GitLabMergeRequestEvent = serde_json::from_str(json).unwrap();
        println!("{:#?}", event);
    }
}
