# GCP Credentials

You can put your service account keys here for them to be used in deployment.

Get your key from GCP panel at https://console.cloud.google.com/iam-admin/serviceaccounts

Service Account -> Keys -> Add Key -> Create new key -> JSON

Default key name is `service_account.json`, this can be modified in the [Makefile](../Makefile).

- [service_account.json](./service_account.json) - default credentials for the service account to use in deployment.
- [`SECRET_STATE_ARCHIVE_KEY`](./SECRET_STATE_ARCHIVE_KEY) - [📃] base64 encoded AES256 key to encrypt and decrypt .tfstate files.
- [`SECRET_CSP_HETZNER`](./SECRET_CSP_HETZNER) - [📃] Hetzner token for deploying a server.

For ENV [📃] secrets values can be placed in files in this directory for automatic exporting to env during deployment.

Example of a file that will be pulled to env vars:

File name: `SECRET_CSP_HETZNER`
File contents:
```
hetzner_token_123
```

Will export a variable to env like so `SECRET_CSP_HETZNER=hetzner_token_123`
