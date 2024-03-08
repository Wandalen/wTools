# Provider for resource creation
provider "google" {
  project = var.PROJECT_ID
}

# Artifact Registry block
resource "google_artifact_registry_repository" "container-images-repo" {
  # Location for the repository
  location      = var.REGION
  project       = var.PROJECT_ID
  repository_id = var.REPO_NAME
  description   = "Docker image registry for the Learn Together web-site"
  # Format of the repository. We are using Docker.
  format        = "DOCKER"
}
