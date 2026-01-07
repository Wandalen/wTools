# --------------------------------------------------------------------------------------------------
# Deploy
# --------------------------------------------------------------------------------------------------

## Required

# KEYS
# Google Cloud
SECRET_STATE_ARCHIVE_KEY="<To generate use: openssl rand -hex 20>"
# Path to the service account credentials
GOOGLE_SE_CREDS_PATH="secret/service_account.json"
# Project id for deployed resources | Can be set in .secret/-secret.sh
PROJECT_ID="driven-era-477418-q2"

# SSH keys | To generate use ssh-keygen -t ed25519 -f ~/.ssh/rsa_key -C "rsa_key"
SECRET_RSA_PRIVATE_KEY_PATH="secret/rsa_key"
SECRET_RSA_PUBLIC_KEY_PATH="secret/rsa_key.pub"

# Default project name (Should not consists "-" or spaces) / `iron_site`
PROJECT_NAME="name_of_the_project"

# Specifies where to deploy the project. Possible values: `hetzner`, `gce`, `aws`
CSP="hetzner"

# Secret hetzner key | Required if CSP="hetzner" | Get from https://docs.hetzner.com/cloud/api/getting-started/generating-api-token/
SECRET_CSP_HETZNER="<Get from https://docs.hetzner.com/cloud/api/getting-started/generating-api-token/>"

# For CSP="aws"
SECRET_AWS_ACCESS_KEY_ID="secret_aws_access_key_id"
SECRET_AWS_ACCESS_KEY="secret_aws_access_key"

# Deployment mode | dev, prod
DEPLOYMENT_MODE="dev"

##  Optional Parameters
# Google cloud region
REGION=
# Artifact Repository name for pushing the Docker images | Should not have "_"
REPO_NAME=
# Pushed image name | Can have "_"
IMAGE_NAME=
# Helper var for tagging local image
TAG=
# Zone location for the resource
ZONE=
# Cloud Storage bucket name | Should not have "_"
BUCKET_NAME=
# Base terraform directory
TF_DIR=


# --------------------------------------------------------------------------------------------------
# Backend
# --------------------------------------------------------------------------------------------------

# Cookie secret variable
COOKIE_SECRET="<To generate use: openssl rand -hex 32>"

# --------------------------------------------------------------------------------------------------
# Frontend
# --------------------------------------------------------------------------------------------------

# Base URL of your backend API (all HTTP requests are sent here) | /api | http://127.0.0.1:8080
VITE_API_BASE_URL="/api"
