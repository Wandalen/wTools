# Specifies region location that's  used for all GCP recources
variable "REGION" {
  description = "region of the resources"
}

# Project id where all resources will be created 
variable "PROJECT_ID" {
  description = "project id for the resources"
}

# Artifact Registry repository name  
variable "REPO_NAME" {
  description = "artifact registry name"
}

# Name of the docker image to pull
variable "IMAGE_NAME" {
  description = "name of the webapp image"
}

# Google Cloud Platform credentials
data "local_sensitive_file" "service_account_creds" {
  filename = "${path.module}/../../key/-service_account.json"
}

# Private key for SSH connection
data "local_sensitive_file" "ssh_private_key" {
  filename = "${path.module}/../../key/-rsa_ssh_key"
}

# Public key for SSH connection
data "local_sensitive_file" "ssh_public_key" {
  filename = "${path.module}/../../key/-rsa_ssh_key.pub"
}
