# Specifies region location that will be used for all recources
variable "REGION" {
  description = "region of the resources"
}

# Specifies zone in the region that will be used for GCE instance
variable "ZONE" {
  description = "zone of the resources"
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


# Templated cloud-init file for providing vars to the boot script
data "template_file" "script" {
  template = "${file("${path.module}/templates/cloud-init.tpl")}"

  vars = {
    location = "${var.REGION}"
    project_id = "${var.PROJECT_ID}"
    repo_name = "${var.REPO_NAME}"
    image_name = "${var.IMAGE_NAME}"
  }
}

# Rendered cloud-init file for startup configurations
data "cloudinit_config" "conf" {
  gzip = false
  base64_encode = false

  part {
    content_type = "text/cloud-config"
    content = "${data.template_file.script.rendered}"
  }
}
