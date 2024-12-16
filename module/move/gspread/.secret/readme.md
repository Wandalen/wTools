# Getting API Keys for OAuth Authentication

Follow these steps to create and configure your OAuth credentials for using Google APIs.

## 1. Create API Credentials

1. Go to the [Google API Console](https://console.developers.google.com/).
2. From the projects list, select an existing project or create a new one.
3. In the left side menu, select **APIs & Services**.
4. On the left menu, click **Credentials**.
5. Click **Create Credentials** and select **OAuth client ID**.
6. In the **Application type** section, select **Desktop app**.
7. Provide an appropriate name for your client ID (e.g., "Gspread OAuth Client").
8. Click **Create**.

Once the credential is created, you will receive a **Client ID** and **Client Secret**. These are required for accessing the API.

## 2. Store Your Credentials

Save the **Client ID** and **Client Secret** in a `.env` within a `.secret` directory. The file should look like this:

```bash
CLIENT_ID=YOUR_CLIENT_ID
CLIENT_SECRET=YOUR_SECRET_KEY
```

## 3. Why do we need it?

After executing each command, you need to grant the GSPREAD program access to the Google API. You will receive a link that begin with 'Please direct your browser to https://....' that will redirect you to your browser, where you must authorize the access. You will need to select the appropriate Google account that has the credentials for the application. The **CLIENT_ID** and **CLIENT_SECRET** are set up to do this process.

## 4. Troubleshooting

If you encounter a page displaying an error instead of the Google account selection screen, it is likely that you need to add **AUTH_URI** or **TOKEN_URI** to the .env file. In this case, all four secrets are required. To retrieve them, download the API key you created in JSON format. Open the file and copy the necessary keys into the .env file. After making these changes, your .env file should look like this:

```bash
CLIENT_ID=YOUR_CLIENT_ID
CLIENT_SECRET=YOUR_SECRET_KEY
AUTH_URI=YOUR_AUTH_URI
TOKEN_URI=YOUR_TOKEN_URI
``` 

If you still get some issues, follow [Google OAuth Documentation](https://developers.google.com/identity/protocols/oauth2/).