# Terraform

This directory contains [Compute Engine](gce/) and [Artifact Registry](gar/) terraform instructions for deploying the web app.

- [gar/](gar/) - Directory contains all terraform resource declarations for creating a repository.
  - [main.tf](./main.tf) - Resources.
  - [outputs.tf](./outputs.tf) - Information to output after the creation of the resources.
  - [variables.tf](./variables.tf) - Configurations for the resources to create.
  - [.tfstate file](./terraform.tfstate) - Current state of GCP to help terraform correctly apply changes.
- [gce/](gce/) - Directory contains all terraform resource declarations for creating a Compute Engine instance.
  - [main.tf](./main.tf) - Resources.
  - [outputs.tf](./outputs.tf) - Information to output after the creation of the resources.
  - [variables.tf](./variables.tf) - Configurations for the resources to create.
  - [.tfstate file](./terraform.tfstate) - Current state of GCP to help terraform correctly apply changes.
  - [templates](./templates/) - Contains templates to be used for resource creation.
    - [templates/cloud-init.tpl](./templates/cloud-init.tpl) - Cloud-init script template to start docker container containing the webapp.

To push an image to be deployed you need to have a [../Dockerfile](../Dockerfile) in the the same directory as your [../Makefile](../Makefile). 

[Compute Engine](gce/) is dependant on [Artifact Registry](gar/) so it's required to create [Artifact Registry](gar/) resources first.
