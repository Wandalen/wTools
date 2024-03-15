provider "aws" {
  region = "eu-west-3"
}

# Search for Ubuntu 22.04 image to run on the instance
data "aws_ami" "ubuntu" {
  most_recent = true

  filter {
    name   = "name"
    values = ["ubuntu/images/hvm-ssd/ubuntu-jammy-22.04-amd64-server-*"]
  }
  owners = ["amazon"]
}

# Security group for the instance to allow for http and ssh connections
resource "aws_security_group" "allow_http_ssh" {
  name        = "allow_http"
  description = "Allow http inbound traffic"


  # Allows incoming requests on port 80
  ingress {
    description = "http"
    from_port   = 80
    to_port     = 80
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]

  }
  # Allows incomming requests on port 22
  ingress {
    description = "ssh"
    from_port   = 22
    to_port     = 22
    protocol    = "tcp"
    cidr_blocks = ["0.0.0.0/0"]

  }
  # Allows outgoing requests to any host on any port
  egress {
    from_port   = 0
    to_port     = 0
    protocol    = "-1"
    cidr_blocks = ["0.0.0.0/0"]
  }


  tags = {
    Name = "allow_http_ssh"
  }
}

# EC2 instance itself
resource "aws_instance" "web" {
  ami             = data.aws_ami.ubuntu.id
  instance_type   = "t2.micro"
  security_groups = ["${aws_security_group.allow_http_ssh.name}"]

  associate_public_ip_address = true

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

  user_data_replace_on_change = true
}

# Static IP address for the instace that will persist on restarts and redeploys
resource "aws_eip" "static" {
  instance = aws_instance.web.id
  domain   = "vpc"
}
