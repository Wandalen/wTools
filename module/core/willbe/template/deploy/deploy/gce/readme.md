# Compute Engine

Directory contains all terraform resource declarations for creating a Compute Engine instance.

- [main.tf](./main.tf) - Resources.
- [outputs.tf](./outputs.tf) - Information to output after the creation of the resources.
- [variables.tf](./variables.tf) - Configurations for the resources to create.
- [.tfstate file](./terraform.tfstate) - Current state of GCP to help terraform correctly apply changes.
- [templates](./templates/) - Contains templates to be used for resource creation.
  - [templates/cloud-init.tpl](./templates/cloud-init.tpl) - Cloud-init script template to start docker container containing the webapp.

## Initialization

Run `terraform init` to validate all resources and download required modules.

## Planning

Run `terraform plan` to review changes to be made by terraform.

## Applying

Run `terraform apply` to review changes to be made by terraform and create/modify resources.

## Destroying

Run `terraform destroy` to destroy created resources.
