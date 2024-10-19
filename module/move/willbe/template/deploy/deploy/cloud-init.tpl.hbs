#cloud-config

write_files:
- path: /root/service_account.json
  permissions: 0600
  owner: root
  content: |
    ${service_account_creds}
- path: /root/init.sh
  permissions: 0700
  owner: root
  content: |
    # Configure env for redeploy script
    echo "DOCKER_IMAGE=${location}-docker.pkg.dev/${project_id}/${repo_name}/${image_name}" >> /etc/environment
    echo "DOCKER_IMAGE_NAME=${image_name}" >> /etc/environment

    # Install docker
    apt update
    apt install apt-transport-https ca-certificates curl software-properties-common -y
    curl -fsSL https://download.docker.com/linux/ubuntu/gpg | apt-key add -
    add-apt-repository "deb [arch=amd64] https://download.docker.com/linux/ubuntu $(lsb_release -cs) stable"
    apt update
    apt install docker-ce -y
    # Install gcloud CLI
    curl https://packages.cloud.google.com/apt/doc/apt-key.gpg | gpg --dearmor -o /usr/share/keyrings/cloud.google.gpg
    echo "deb [signed-by=/usr/share/keyrings/cloud.google.gpg] https://packages.cloud.google.com/apt cloud-sdk main" | tee -a /etc/apt/sources.list.d/google-cloud-sdk.list
    apt-get update
    apt-get install -y google-cloud-cli
    # Configure docker with gcloud
    gcloud auth activate-service-account --key-file=/root/service_account.json
    gcloud auth configure-docker ${location}-docker.pkg.dev --quiet


runcmd:
- nohup /root/init.sh > /var/log/deploy-init.log 2>&1 &
