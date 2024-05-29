# Provider for resource creation
provider "google" {
  project = var.PROJECT_ID
}


# Storage bucket itself
resource "google_storage_bucket" "tfstate-storage" {
  name                        = var.BUCKET_NAME
  location                    = var.REGION
  # Delete files stored on the bucket when destroying the bucket
  force_destroy               = true
  uniform_bucket_level_access = true
  public_access_prevention    = "enforced"
}


# Name of the bucket that will be created 
variable "BUCKET_NAME" {
  description = "name for the bucket to be created"
}

# Specifies region location that will be used for all recources
variable "REGION" {
  description = "region of the resources"
}

# Project id where all resources will be created 
variable "PROJECT_ID" {
  description = "project id for the resources"
}
