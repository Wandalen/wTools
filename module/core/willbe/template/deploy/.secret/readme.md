## Secrets & Configuration

This project uses a **single secrets file** for both local development and deployment.  
All secrets are **never committed** to the repository and are stored outside of git.

---

## Directory Structure

```text
.secret/
├── readme.md                   # secrets documentation and setup instructions (this file)
├── secret.template.sh          # reference template for all required secret variables
├── -secret.sh                  # main secrets file (environment variables)
├── service_account.json        # Google Cloud service account
├── rsa_key                     # RSA private key
└── rsa_key.pub                 # RSA public key
```

- The entire `.secret/` directory is gitignored
- Files prefixed with `-` are also automatically ignored

---

## 1. Deployment Variables

The following variables are required for infrastructure provisioning and deployment.

| Required  | Variable          | Description                   | Notes                             |
|---------- |----------         |-------------                  |-------                            |
| Yes       | SECRET_STATE_ARCHIVE_KEY | Terraform state encryption key | Generate with `openssl rand -hex 20` |
| Yes       | GOOGLE_SE_CREDS_PATH | Path to GCP service account JSON | `secret/service_account.json` |
| Yes       | PROJECT_ID        | Google Cloud project ID       | Used for deployed resources       |
| Yes       | SECRET_RSA_PRIVATE_KEY_PATH | Path to RSA private key | SSH/RSA key for infrastructure access |
| Yes       | SECRET_RSA_PUBLIC_KEY_PATH | Path to RSA public key | Public counterpart of RSA key   |
| Yes       | PROJECT_NAME      | Project name                  | Must not contain `-` or spaces    |
| Yes       | CSP               | Cloud provider                | `hetzner`, `gce`, `aws`           |
| Conditional | SECRET_CSP_HETZNER | Hetzner API token          | Required if `CSP=hetzner`         |
| Conditional | SECRET_AWS_ACCESS_KEY_ID | AWS access key ID    | Required if `CSP=aws`             |
| Conditional | SECRET_AWS_ACCESS_KEY | AWS secret access key   | Required if `CSP=aws`             |
| Yes       | DEPLOYMENT_MODE   | Deployment mode               | `dev` or `prod`                   |
| No        | REGION            | Cloud region                  | Optional, provider-specific       |
| No        | REPO_NAME         | Artifact repository name      | Must not contain `_`              |
| No        | IMAGE_NAME        | Docker image name             | `_` allowed                       |
| No        | TAG               | Docker image tag              | Helper variable                   |
| No        | ZONE              | Cloud zone                    | Provider-specific                 |
| No        | BUCKET_NAME       | Cloud storage bucket name     | Must not contain `_`              |
| No        | TF_DIR            | Base Terraform directory      | Advanced usage                    |

Refer to `secret.template.sh` for the canonical list and inline comments.


---

## 2. Backend Variables

Environment variables used by the backend server:

| Required  | Variable          | Description                   | Notes                             |
|---------- |----------         |-------------                  |-------                            |
| Yes       | COOKIE_SECRET     | Cookie signing secret         | Generate a strong random value    |

Additional backend variables can be added as required by the application.

---

## 3. Frontend Variables (Vite)

Variables available at build time for the frontend:

| Required  | Variable          | Description                   | Example                           |
|---------- |----------         |-------------                  |---------                          |
| Yes       | VITE_API_BASE_URL | Base URL of the backend API   | `/api`, `http://127.0.0.1:8080`   |

---


## Usage

### Load Secrets into the Shell

```bash
source .secret/-secret.sh
```

### Deployment

- `Makefile.deploy` automatically:
  - sources `.secret/-secret.sh`
  - validates required variables
  - exports variables for Terraform and Docker

---

## Security Notes

- **Never commit** `.secret/-secret.sh` or any files from `.secret/`
- Restrict file permissions:

```bash
chmod 600 .secret/-secret.sh
chmod 600 .secret/*
```

- Rotate secrets regularly
- For production, consider using a managed secrets solution:
  - HashiCorp Vault
  - AWS Secrets Manager
  - Google Secret Manager

---

## Migration from `.env`

```bash
# Create the secrets file
touch .secret/-secret.sh

# Copy variables from .env
cat .env | sed 's/^/export /' >> .secret/-secret.sh

# Add required deployment variables from secret.template.sh
```

---

## Google Cloud Service Account (JSON Keys)

This project requires a **Google Cloud service account JSON key** for Terraform state storage, Artifact Registry, and compute resources.

### Create a Service Account

1. Open **Google Cloud Console**
2. Select your project (or create a new one)
3. Navigate to **IAM & Admin → Service Accounts**
4. Click **Create Service Account**
5. Set a name (e.g. `terraform-deploy`)

### Required Permissions

Grant the following roles to the service account:

- **Storage Admin** — manage GCS buckets and Terraform state
- **Artifact Registry Administrator** — push/pull Docker images
- **Compute Admin** — manage VM instances
- **Service Account User** — allow Terraform to act as the account

### Download JSON Key

1. Open the created service account
2. Go to the **Keys** tab
3. Click **Add Key → Create new key**
4. Select **JSON** and download the file
5. Move it into the project:

```bash
mv ~/Downloads/your-project-*.json .secret/service_account.json
chmod 600 .secret/service_account.json
```

6. Set the path in `.secret/-secret.sh`:

```bash
GOOGLE_SE_CREDS_PATH=".secret/service_account.json"
```

---

## Hetzner Cloud API Token

A Hetzner API token is required when `CSP="hetzner"`.

### Create Token

1. Go to **Hetzner Cloud Console**
2. Select your project (or create one)
3. Navigate to **Security → API Tokens**
4. Click **Generate API Token**
5. Set a name (e.g. `terraform-deploy`)
6. Permissions: **Read & Write**

⚠️ The token is shown **only once** — copy it immediately.

### Add to Secrets File

```bash
SECRET_CSP_HETZNER="your-hetzner-api-token"
```

---

## AWS Access Keys

AWS credentials are required when `CSP="aws"`.

### Create IAM User

1. Open **AWS Console → IAM**
2. Go to **Users → Create user**
3. Set a username (e.g. `terraform-deploy`)
4. Enable **Programmatic access**

### Permissions

Attach the following policies:

- **AmazonEC2FullAccess**
- **AmazonS3FullAccess**
- **IAMFullAccess** (or a restricted custom policy)

### Create Access Keys

1. Open the IAM user
2. Go to **Security Credentials**
3. Click **Create access key**
4. Select **Other / CLI usage**
5. Save the keys securely

### Add to Secrets File

```bash
SECRET_AWS_ACCESS_KEY_ID="your-access-key-id"
SECRET_AWS_ACCESS_KEY="your-secret-access-key"
```

---

## Generating SSH / RSA Keys Locally

The deployment process requires an RSA (or Ed25519) key pair for secure access and infrastructure operations.

### Generate SSH Key Pair (Recommended: Ed25519)

```bash
ssh-keygen -t ed25519 -C "deployment-key" -f .secret/rsa_key
```

This command will generate:

- `.secret/rsa_key` — private key
- `.secret/rsa_key.pub` — public key

### Alternative: Generate RSA Key Pair

If Ed25519 is not supported, you can generate an RSA key instead:

```bash
ssh-keygen -t rsa -b 4096 -C "deployment-key" -f .secret/rsa_key
```

### Set File Permissions

```bash
chmod 600 .secret/rsa_key
chmod 644 .secret/rsa_key.pub
```

### Configure Paths

Ensure the following variables are set in `.secret/-secret.sh`:

```bash
SECRET_RSA_PRIVATE_KEY_PATH="secret/rsa_key"
SECRET_RSA_PUBLIC_KEY_PATH="secret/rsa_key.pub"
```

---

## Verification

```bash
ls -lh .secret/
make -f Makefile.deploy all
```
