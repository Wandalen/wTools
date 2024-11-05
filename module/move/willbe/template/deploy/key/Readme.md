# Keys

This document provides a concise example of an environment configuration script, used to set up environment variables for a project.
These variables configure application behavior without altering the code.

- [Keys](#keys)
  - [Examples](#examples)
    - [`-gcp.sh`](#-gcpsh)
    - [`-hetzner.sh`](#-hetznersh)
    - [`-aws.sh`](#-awssh)
  - [How to Run](#how-to-run)
  - [Retrieving keys](#retrieving-keys)
    - [How to get `service_account.json`](#how-to-get-service_accountjson)
    - [How to get `SECRET_STATE_ARCHIVE_KEY`](#how-to-get-secret_state_archive_key)
    - [How to get `SECRET_CSP_HETZNER`](#how-to-get-secret_csp_hetzner)
    - [How to get `SECRET_AWS_ACCESS_KEY_ID` and `SECRET_AWS_ACCESS_KEY`](#how-to-get-secret_aws_access_key_id-and-secret_aws_access_key)


## Examples

### `-gcp.sh`

Contents example for the file `-gcp.sh`. This is a required configuration for all deploy targets.

```bash
#!/bin/bash
CSP=gce
SECRET_STATE_ARCHIVE_KEY=qK1/4m60aZvclYi4bZFeBl8GxpyWcJ2iEevHN+uMy7w=

FILE_PATH="$( realpath -qms "${BASH_SOURCE[0]:-$PWD}" )"
DIR_PATH="${FILE_PATH%/*}"
head -c -1 << EOF > ${DIR_PATH}/-service_account.json
{
  // Your service_account information
}
EOF
```

- `CSP`: (Optional) Specifies deployment to GCE.
- `SECRET_STATE_ARCHIVE_KEY`: Base64 encoded AES256 key to encrypt and decrypt .tfstate files.
- `-service_account.json`: Default credentials for the service account to use in deployment.

### `-hetzner.sh`

Contents example for the file `-hetzner.sh`:

```bash
CSP=hetzner
SECRET_CSP_HETZNER=your_token_here
```

- `CSP`: Specifies deployment to Hetzner.
- `SECRET_CSP_HETZNER`: Hetzner token for deploying a server.

### `-aws.sh`

Contents example for the file `-aws.sh`:

```bash
CSP=aws
SECRET_AWS_ACCESS_KEY_ID=aws_credentials_here
SECRET_AWS_ACCESS_KEY=aws_credentials_here
```

- `CSP`: Specifies deployment to AWS.
- `SECRET_AWS_ACCESS_KEY_ID`: Access Key ID from AWS Credentials. Created at the same time as the Access Key itself.
- `SECRET_AWS_ACCESS_KEY`: Access Key for AWS API. Has to be accompanied with respectful Access Key ID.

## How to Run

To apply these variables to your current shell session, use:

```bash
. ./key/-gcp.sh
. ./key/-hetzner.sh
```

This command sources the script, making the variables available in your current session and allowing deployment to Hetzner.
Ensure `-env.sh` is in the `key` directory relative to your current location.

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
