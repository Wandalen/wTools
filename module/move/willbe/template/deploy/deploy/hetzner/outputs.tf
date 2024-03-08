locals {
  ip = hcloud_server.uaconf.ipv4_address
}

# Output that we get after applying.
# IPv4 address of the created GCE instance.
output "ipv4" {
  description = "The public IP address of the deployed instance"
  value       = local.ip
}

# Output link to the deployed website.
output "http" {
  description = "The public IP address of the deployed instance"
  value       = format("http://%s/", local.ip)
}
