# jira-gitlab-validating-webhook

A simple webhook service for automatically closing MRs in Gitlab that do not contain a reference to a JIRA ticket. Adds a courtesy comment upon MR close.

## configuration

The following environment variables are required:

| Variable Name   | Example   |
|---|---|
| GITLAB_HOST  | gitlab.com  |
| GITLAB_TOKEN  |   |
| JIRA_HOST  | issues.redhat.com  |
| JIRA_TOKEN | |
| JIRA_USERNAME | |
| JIRA_PASSWORD | |

One of `JIRA_TOKEN` or `JIRA_USERNAME + JIRA_PASSWORD` is required for JIRA authentication.

## developing

1) install `rust`
2) `cargo run` or `cargo test`

A dockerfile is provided for easy deployment. You need to configure gitlab to point to your service and add `/webhook` to the path. **note:** if your GitLab instance is internal you may need to add a custom CA into the image.
