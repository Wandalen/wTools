# Specifies region location that will be used for all recources
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
