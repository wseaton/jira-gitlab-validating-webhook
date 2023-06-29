use axum::{
    extract::State,
    routing::{get, post},
    Json, Router, Server,
};

use jira_query::{Auth, JiraInstance};
use regex::Regex;
use std::net::SocketAddr;
use std::sync::Arc;

mod gitlab;
use gitlab::GitLabMergeRequestEvent;

use tower_http::trace::TraceLayer;
use tracing::{debug, error, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use once_cell::sync::Lazy;

static GITLAB_HOST: Lazy<String> =
    Lazy::new(|| std::env::var("GITLAB_HOST").expect("GITLAB_HOST must be set"));

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "jira-gitlab-validating-webhook=debug,jira_query=debug,tower_http=debug,axum::rejection=trace,info".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let jira = git_jira_instance();

    let app = Router::new()
        .route("/webhook", post(process_webhook))
        .layer(TraceLayer::new_for_http())
        .route("/healthz", get(health_check))
        .with_state(jira); // replace with your secret token

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));

    info!("Listening on {}", addr);
    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("server failed");

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}

async fn process_webhook(
    State(jira): State<Arc<JiraInstance>>,
    Json(event): Json<GitLabMergeRequestEvent>,
) -> Result<String, ()> {
    info!(
        "Received MR event with state: {}",
        event.object_attributes.state
    );
    if event.object_attributes.state != "opened" {
        info!("Merge request {} is not open", event.object_attributes.iid);
        return Ok("Merge request is not being opened, ignoring.".to_string());
    }

    if let Some(ticket) = extract_jira_ticket(&event.object_attributes.source_branch) {
        match jira.issue(&ticket).await {
            Ok(issue) => {
                let output = format!("Valid JIRA ticket in branch name: {}", issue.key);
                info!(output);
                Ok(output)
            }
            Err(_) => {
                let output = format!("Invalid JIRA ticket in branch name: {}", ticket);
                info!(output);
                Ok(output)
            }
        }
    } else if let Some(ticket) = extract_jira_ticket(&event.object_attributes.description) {
        match jira.issue(&ticket).await {
            Ok(issue) => Ok(format!("Valid JIRA ticket in description: {}", issue.key)),
            Err(_) => Ok(format!("Invalid JIRA ticket in description: {}", ticket)),
        }
    } else {
        info!("No JIRA ticket found in the branch name or description");
        // automatically close the merge request
        match close_gitlab_mr(event.project.id.unwrap(), event.object_attributes.iid).await {
            Ok(msg) => {
                match comment_close_messge_on_mr(
                    event.project.id.unwrap(),
                    event.object_attributes.iid,
                )
                .await
                {
                    Ok(_) => {}
                    Err(_) => {
                        error!("Failed to comment on merge request");
                    }
                }
                Ok(msg)
            }
            Err(_) => Err(()),
        }
    }
}

fn extract_jira_ticket(text: &str) -> Option<String> {
    let re = Regex::new(r"[A-Z]+-\d+").unwrap();
    re.find(text).map(|m| m.as_str().to_string())
}

async fn close_gitlab_mr(project_id: u64, mr_id: u64) -> Result<String, ()> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://{gitlab_host}/api/v4/projects/{project_id}/merge_requests/{mr_id}",
        gitlab_host = *GITLAB_HOST
    );
    let response = client
        .put(&url)
        .header(
            "PRIVATE-TOKEN",
            std::env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN not set"),
        )
        .query(&[("state_event", "close")])
        .send()
        .await
        .unwrap();

    let status = response.status();

    if status.is_success() {
        info!("Merge request {} closed", mr_id);
        Ok(format!("Merge request {} closed", mr_id))
    } else {
        error!("Failed to close merge request {}: {}", mr_id, status);
        debug!("Response: {:?}", response.text().await);
        Err(())
    }
}

async fn comment_close_messge_on_mr(project_id: u64, mr_id: u64) -> Result<(), ()> {
    let client = reqwest::Client::new();
    let url = format!(
        "https://{gitlab_host}/api/v4/projects/{project_id}/merge_requests/{mr_id}/notes",
        gitlab_host = *GITLAB_HOST
    );
    let response = client
        .post(&url)
        .header(
            "PRIVATE-TOKEN",
            std::env::var("GITLAB_TOKEN").expect("GITLAB_TOKEN not set"),
        )
        .query(&[("body", "This merge request has been closed automatically because it does not contain a valid JIRA ticket in the branch name or description.")])
        .send()
        .await
        .unwrap();

    let status = response.status();
    if status.is_success() {
        info!("Comment on MR {} done.", mr_id);
    } else {
        error!("Failed to comment on MR {}: {}", mr_id, status);
        debug!("Response: {:?}", response.text().await);
    }

    Ok(())
}

fn git_jira_instance() -> Arc<JiraInstance> {
    let jira_host = std::env::var("JIRA_HOST").expect("JIRA_HOST not set");

    let jira_ = JiraInstance::at(jira_host).expect("Failed to create JIRA instance");

    let jira = if let Ok(api_key) = std::env::var("JIRA_TOKEN") {
        jira_.authenticate(Auth::ApiKey(api_key))
    } else if let (Ok(user), Ok(password)) = (
        std::env::var("JIRA_USERNAME"),
        std::env::var("JIRA_PASSWORD"),
    ) {
        jira_.authenticate(Auth::Basic { user, password })
    } else {
        panic!("JIRA_TOKEN or JIRA_USERNAME and JIRA_PASSWORD not set");
    };

    Arc::new(jira)
}
