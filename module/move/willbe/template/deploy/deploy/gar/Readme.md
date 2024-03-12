# Artifact Registry

Directory contains all terraform resource declarations for creating a repository.

- [main.tf](./main.tf) - Resources.
- [outputs.tf](./outputs.tf) - Information to output after the creation of the resources.
- [variables.tf](./variables.tf) - Configurations for the resources to create.
- [.tfstate file](./terraform.tfstate) - Current state of GCP to help terraform correctly apply changes.

## Initialization

Run `terraform init` to validate all resources and download required modules.

## Planning

Run `terraform plan` to review changes to be made by terraform.

## Applying

Run `terraform apply` to review changes to be made by terraform and create/modify resources.

## Destroying

Run `terraform destroy` to destroy created resources.
