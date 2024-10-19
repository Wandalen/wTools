# Deploy credentials

A list of all keys you'd need to deploy your project on different hosts.

- [Deploy credentials](#deploy-credentials)
  - [Files](#files)
  - [Env vars](#env-vars)
  - [Retrieving keys](#retrieving-keys)
    - [How to get `service_account.json`](#how-to-get-service_accountjson)
    - [How to get `SECRET_STATE_ARCHIVE_KEY`](#how-to-get-secret_state_archive_key)
    - [How to get `SECRET_CSP_HETZNER`](#how-to-get-secret_csp_hetzner)
    - [How to get `SECRET_AWS_ACCESS_KEY_ID` and `SECRET_AWS_ACCESS_KEY`](#how-to-get-secret_aws_access_key_id-and-secret_aws_access_key)

## Files

All secrets can be provided as files in current directory:

- [service_account.json](./service_account.json) - default credentials for the service account to use in deployment.
- [rsa_ssh_key](./rsa_ssh_key) - SSH Private key that will be used for redeployment.
- [rsa_ssh_key.pub](./rsa_ssh_key.pub) - SSH Private key that will be used for redeployment.
- [`SECRET_STATE_ARCHIVE_KEY`](./SECRET_STATE_ARCHIVE_KEY) - [📃] base64 encoded AES256 key to encrypt and decrypt .tfstate files.
- [`SECRET_CSP_HETZNER`](./SECRET_CSP_HETZNER) - [📃] Hetzner token for deploying a server.
- [`SECRET_AWS_ACCESS_KEY_ID`](./SECRET_AWS_ACCESS_KEY_ID) - [📃] Access Key ID from AWS Credentials. Created at the same time as the Access Key itself.
- [`SECRET_AWS_ACCESS_KEY`](./SECRET_AWS_ACCESS_KEY) - [📃] Access Key for AWS API. Has to be accompanied with respectful Access Key ID.

## Env vars

Some secrets can be presented as an env var:

- [`SECRET_STATE_ARCHIVE_KEY`](./SECRET_STATE_ARCHIVE_KEY) - [📃] base64 encoded AES256 key to encrypt and decrypt .tfstate files.
- [`SECRET_CSP_HETZNER`](./SECRET_CSP_HETZNER) - [📃] Hetzner token for deploying a server.
- [`SECRET_AWS_ACCESS_KEY_ID`](./SECRET_AWS_ACCESS_KEY_ID) - [📃] Access Key ID from AWS Credentials. Created at the same time as the Access Key itself.
- [`SECRET_AWS_ACCESS_KEY`](./SECRET_AWS_ACCESS_KEY) - [📃] Access Key for AWS API. Has to be accompanied with respectful Access Key ID.

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

### How to get `SECRET_AWS_ACCESS_KEY_ID` and `SECRET_AWS_ACCESS_KEY`

Can be created in your AWS Console on the following the link:
https://console.aws.amazon.com/iam/home?#security_credential

Access Keys -> Create Access Key -> Other -> Next -> Fill key description -> Create Access Key

The Access Key ID will be always available to view, but secret access key is only visible after the key creation.

You need to have credential creation permissions on your AWS account.

An example of permissions to give to an account managing the deployment can be found here:
https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_policies_examples_aws_my-sec-creds-self-manage-no-mfa.html

You also need to give [AmazonEC2FullAccess](https://docs.aws.amazon.com/aws-managed-policy/latest/reference/AmazonEC2FullAccess.html)
permission for your user to create an EC2 instance.
