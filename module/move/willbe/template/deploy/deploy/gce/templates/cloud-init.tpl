#cloud-config

users:
- name: ${image_name}
  uid: 2000

write_files:
- path: /etc/systemd/system/${image_name}.service
  permissions: 0644
  owner: root
  content: |
    [Unit]
    Description=Start the Learn Together ${image_name} docker container
    Wants=gcr-online.target
    After=gcr-online.target

    [Service]
    Environment="HOME=/home/${image_name}"
    ExecStartPre=/usr/bin/docker-credential-gcr configure-docker --registries=${location}-docker.pkg.dev
    ExecStart=/usr/bin/docker run -d -p 80:80 --name=${image_name} ${location}-docker.pkg.dev/${project_id}/${repo_name}/${image_name}

runcmd:
- systemctl daemon-reload
- systemctl start ${image_name}.service