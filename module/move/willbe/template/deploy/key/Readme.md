# Deploy credentials

A list of all keys you'd need to deploy your project on different hosts.

- [Deploy credentials](#deploy-credentials)
  - [Files](#files)
  - [Env vars](#env-vars)
  - [Retrieving keys](#retrieving-keys)
    - [How to get `service_account.json`](#how-to-get-service_accountjson)
    - [How to get `SECRET_STATE_ARCHIVE_KEY`](#how-to-get-secret_state_archive_key)
    - [How to get `SECRET_CSP_HETZNER`](#how-to-get-secret_csp_hetzner)


## Files

All secrets can be provided as files in current directory:

- [service_account.json](./service_account.json) - default credentials for the service account to use in deployment.
- [`SECRET_STATE_ARCHIVE_KEY`](./SECRET_STATE_ARCHIVE_KEY) - [📃] base64 encoded AES256 key to encrypt and decrypt .tfstate files.
- [`SECRET_CSP_HETZNER`](./SECRET_CSP_HETZNER) - [📃] Hetzner token for deploying a server.

## Env vars

Some secrets can be presented as an env var:

- [`SECRET_STATE_ARCHIVE_KEY`](./SECRET_STATE_ARCHIVE_KEY) - [📃] base64 encoded AES256 key to encrypt and decrypt .tfstate files.
- [`SECRET_CSP_HETZNER`](./SECRET_CSP_HETZNER) - [📃] Hetzner token for deploying a server.

Env vars have a higher priority then the files.

For ENV [📃] secrets values can be placed in files in this directory for automatic exporting to env during deployment.

Example of a file that will be pulled to env vars:

File name: `SECRET_CSP_HETZNER`
File contents:
```
hetzner_token_123
```

Will export a variable to env like so `SECRET_CSP_HETZNER=hetzner_token_123`

## Retrieving keys

Explanation for fetching all required keys.

### How to get `service_account.json`

You can put your service account keys here for them to be used in deployment.

Get your key from GCP panel at https://console.cloud.google.com/iam-admin/serviceaccounts

Service Account -> Keys -> Add Key -> Create new key -> JSON

Default key name is `service_account.json`, this can be modified in the [Makefile](../Makefile).

### How to get `SECRET_STATE_ARCHIVE_KEY`

You can generate this key via multiple ways.

This page on GCP describes some methods you could utilize for generation:

https://cloud.google.com/storage/docs/encryption/using-customer-supplied-keys

### How to get `SECRET_CSP_HETZNER`

This key can be retrieved from your Hetzner dashboard.

Cloud Console -> Security -> API Tokens -> Generate API Token

Fill the token description and all `Read & Write` access, since this key will be used for instance creation.
