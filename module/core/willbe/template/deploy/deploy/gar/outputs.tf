# Output that we get after applying.
# Return name for the created repository for verification.
output "repo_name" {
  description = "Name of the Artifact Registry"
  value       = google_artifact_registry_repository.container-images-repo.name
}
