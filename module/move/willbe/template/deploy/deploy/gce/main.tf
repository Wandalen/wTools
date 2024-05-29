locals {
  # Helper var for formatting docker image name
  image_name     = format("%s-docker.pkg.dev/%s/%s/%s", var.REGION, var.PROJECT_ID, var.REPO_NAME, var.IMAGE_NAME)
  # Helper var for formatting subnetwork for our instance
  subnetwork     = format("projects/%s/regions/%s/subnetworks/default", var.PROJECT_ID, var.REGION)
  instance_name  = format("ltsite-%s", formatdate("YYYYMMDDhhmmss", timestamp()))
}

# Provider for resource creation
provider "google" {
  project = var.PROJECT_ID
}

# Static IP for our GCE instance so we don't lose the address after re-creating the instance.
resource "google_compute_address" "default" {
  name   = "lts-static-ip-address"
  region = var.REGION
}

# GCE instance block.
resource "google_compute_instance" "lts-container-vm" {
  project      = var.PROJECT_ID
  # Instance name
  name         = local.instance_name
  # Instance size. e2-micro is 0.25-2 vCPU & 1GB RAM
  machine_type = "e2-micro"
  zone         = var.ZONE

  # Main disk options
  boot_disk {
    initialize_params {
      # Disk image name. We're using Container-optimised OS (COS).
      image = "projects/cos-cloud/global/images/cos-stable-109-17800-147-15"
      # Disk size in GB. 10GB is allowed minimum.
      size  = 10
      # Disk type. Possible values: pd-standard, pd-ssd, or pd-balanced.
      type  = "pd-balanced"
    }
  }

  network_interface {
    # Subnetwork to use.
    subnetwork = local.subnetwork
    access_config {
      # Network tier for the instance. Possible values: PREMIUM or STANDARD.
      network_tier = "STANDART"
      # Set our static IP for the instance.
      nat_ip = google_compute_address.default.address
    }
  }

  metadata = {
    # Cloud-init startup script for configuring the instance with our docker container.
    user-data = "${data.cloudinit_config.conf.rendered}"
  }

  allow_stopping_for_update = true

  scheduling {
    # Restart on failure.
    automatic_restart   = true
    # Describes maintenance behavior for the instance. Possible values: MIGRATE or TERMINATE.
    on_host_maintenance = "MIGRATE"
    # Configures whether to allow stopping instance at any moment for reduced cost.
    preemptible         = false
    # Configures spot instance. Possible values: SPOT or STANDARD. 
    provisioning_model  = "STANDARD"
  }

  # Configues service account scopes.
  service_account {
    scopes = [
      # Scope for reading data from buckets/Artifact Registry.
      "https://www.googleapis.com/auth/devstorage.read_only",
      # Logging and etc scopes
      "https://www.googleapis.com/auth/logging.write",
      "https://www.googleapis.com/auth/monitoring.write",
      "https://www.googleapis.com/auth/service.management.readonly",
      "https://www.googleapis.com/auth/servicecontrol",
      "https://www.googleapis.com/auth/trace.append"
    ]
  }

  # Tags for the instance.
  # `http-server` automatically allows all http traffic on port 80.
  # Use `https-server` for https traffic on port 443.
  tags = ["http-server"]
}
