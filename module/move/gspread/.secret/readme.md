# Getting API Keys for OAuth Authentication

Follow these steps to create and configure your OAuth credentials for using Google APIs.

## 1. Configure Consent Screen

1. Go to the [Google API Console](https://console.developers.google.com/).
2. From the projects list, select an existing project or create a new one.
3. Go to **OAuth consent screen**
4. Choose **Extrenal** User Type
5. Fill **App name**, **User support email** and **Developer contact information**. Click **continue**
6. Click on **ADD OR REMOVE SCOPES**
7. Add **.../auth/userinfo.email** and **.../auth/userinfo.profile** spoces.
8. Finish configuration

## 2. Enable Google Sheets API

1. Go to the [Google API Console](https://console.developers.google.com/).
2. In the left side menu, select **Enabled APIs & Services**.
3. Click on **ENABLE APIS AND SERVICES**
4. Search for **Google Sheets API**
5. Click on **Enable**

## 2. Create API Credentials

1. Go to the [Google API Console](https://console.developers.google.com/).
2. From the projects list, select an existing project or create a new one.
3. In the left side menu, select **APIs & Services**.
4. On the left menu, click **Credentials**.

### 1-1. Service Account
1. Click **Create Credentials** and select **Service account**.
2. Enter a name of your app. Then put on the **Done** button.
3. Click on app email in section **Service Account**. After put on the **Keys**.
4. Create new keys in JSON type.

### 1-2. OAuth client ID
5. Click **Create Credentials** and select **OAuth client ID**.
6. In the **Application type** section, select **Desktop app**.
7. Provide an appropriate name for your client ID (e.g., "Gspread OAuth Client").
8. Click **Create**.

After you will have all required tokens to use application

## 3. Store Your Credentials

For **Service Account**, **Client ID** and **Client Secret** are enough.
For **Service account** use all tokens.

Save the credentials in a `.env` within a `.secret` directory. The file should look like this:

```bash
CLIENT_ID=YOUR_CLIENT_ID
CLIENT_SECRET=YOUR_SECRET_KEY
# other tokens
# ....
```

## 4. Why do we need it?

After executing each command, you need to grant the GSPREAD program access to the Google API. You will receive a link that begin with 'Please direct your browser to https://....' that will redirect you to your browser, where you must authorize the access. You will need to select the appropriate Google account that has the credentials for the application. The tokens are set up to do this process.

## 5. Troubleshooting

### 1-1. OAuth client ID
If you encounter a page displaying an error instead of the Google account selection screen, it is likely that you need to add **AUTH_URI** or **TOKEN_URI** to the .env file. In this case, all four secrets are required. To retrieve them, download the API key you created in JSON format. Open the file and copy the necessary keys into the .env file. After making these changes, your .env file should look like this:

```bash
CLIENT_ID=YOUR_CLIENT_ID
CLIENT_SECRET=YOUR_SECRET_KEY
AUTH_URI=YOUR_AUTH_URI
TOKEN_URI=YOUR_TOKEN_URI
``` 

If you still get some issues, follow [Google OAuth Documentation](https://developers.google.com/identity/protocols/oauth2/).