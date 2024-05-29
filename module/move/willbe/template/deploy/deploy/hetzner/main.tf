terraform {
  # Specifies terraform API provider to use for `hcloud`
  required_providers {
    hcloud = {
      source  = "hetznercloud/hcloud"
      version = "1.45.0"
    }
  }
}

# Configures hcloud provider for deploy
provider "hcloud" {
  # Hetzner API token 
  token = var.HCLOUD_TOKEN
}

# Static IP for the instance
resource "hcloud_primary_ip" "primary_ip" {
  name          = "uaconf-2024-ip"
  datacenter    = "hel1-dc2"
  type          = "ipv4"
  assignee_type = "server"
  auto_delete   = false
}

# Hetzner instance itself
resource "hcloud_server" "uaconf" {
  name        = "uaconf-2024"
  image       = "ubuntu-22.04"
  server_type = "cx11"
  datacenter  = "hel1-dc2"

  public_net {
    ipv4_enabled = true
    ipv4         = hcloud_primary_ip.primary_ip.id
    ipv6_enabled = false
  }

  # Startup script for the instance
  # Installs docker, gcloud CLI, downloads docker images and starts the container
  user_data = templatefile("${path.module}/templates/cloud-init.tpl", {
    location              = "${var.REGION}"
    project_id            = "${var.PROJECT_ID}"
    repo_name             = "${var.REPO_NAME}"
    image_name            = "${var.IMAGE_NAME}"
    service_account_creds = "${replace(data.local_sensitive_file.service_account_creds.content, "\n", "")}"
    timestamp             = "${timestamp()}"
  })
}
