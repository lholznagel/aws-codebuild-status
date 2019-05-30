# AWS-Codebuild-Status

Small rust terminal application that list all [AWS-Codebuild](https://aws.amazon.com/de/codebuild/) projects and their current status.

## Preparation

- Create a new IAM User
- The following permissions are needed:
  - Replace `REGION` and `ACCOUNT`

``` json
{
    "Version": "2012-10-17",
    "Statement": [
        {
            "Sid": "VisualEditor0",
            "Effect": "Allow",
            "Action": [
                "codebuild:BatchGetProjects",
                "codebuild:BatchGetBuilds"
            ],
            "Resource": "arn:aws:codebuild:REGION:ACCOUNT:project/*"
        },
        {
            "Sid": "VisualEditor1",
            "Effect": "Allow",
            "Action": "codebuild:ListBuildsForProject",
            "Resource": "arn:aws:codebuild:REGION:ACCOUNT:project/*"
        },
        {
            "Sid": "VisualEditor2",
            "Effect": "Allow",
            "Action": "codebuild:ListProjects",
            "Resource": "*"
        },
        {
            "Sid": "VisualEditor3",
            "Effect": "Allow",
            "Action": [
                "codecommit:ListBranches",
                "codecommit:GetBranch"
            ],
            "Resource": "arn:aws:codecommit:REGION:ACCOUNT:*"
        }
    ]
}
```

## Usage
Before you can use it the `AWS_ACCESS_KEY_ID`, `AWS_SECRET_ACCESS_KEY` and optional `AWS_REGION` must be set as environment variable, as a file in `~/.aws/config` or `~/.aws/credentials`. As an alternative the IAM instance profile will also work.

Install it with `cargo install aws-codebuild-status`

It is now executable with `aws-codebuild-status`

## Screenshots

### Terminal
[![screenshot](./assets/screenshot_terminal.png)](./assets/screenshot_terminal.png)

### Webpage
[![screenshot](./assets/screenshot_web.png)](./assets/screenshot_web.png)