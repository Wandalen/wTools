# Getting API Keys for OAuth Authentication

Follow these steps to create and configure your OAuth credentials for using Google APIs.

## 1. Create API Credentials

1. Go to the [Google API Console](https://console.developers.google.com/).
2. From the projects list, select an existing project or create a new one.
3. In the left side menu, select **APIs & Services**.
4. On the left menu, click **Credentials**.
5. Click **Create Credentials** and select **OAuth client ID**.
6. In the **Application type** section, select **Desktop app**.
7. Provide an appropriate name for your client ID (e.g., "MyApp OAuth Client").
8. Click **Create**.

Once the credential is created, you will receive a **Client ID** and **Client Secret**. These are required for accessing the API.

## 2. Store Your Credentials

Save the **Client ID** and **Client Secret** in a `.sh` file (e.g., `-env.sh`) within a `key` directory. The file should look like this:

```bash
CLIENT_ID=YOUR_CLIENT_ID
CLIENT_SECRET=YOUR_SECRET_KEY
```

Set also these keys with following values:
```bash
AUTH_URI=https://accounts.google.com/o/oauth2/auth
TOKEN_URI=https://oauth2.googleapis.com/token
```
If you get problems, most likely you will need to change **AUTH_URI** or **TOKEN_URI** to the appropriate one. Try to download your API KEY that you created in JSON format. Then open it and you will see right links. Just copy them and past to file. 
Otherwise, follow [Google OAuth Documentation](https://developers.google.com/identity/protocols/oauth2/) to solve them.
Most likely you will need to change **AUTH_URI** or **TOKEN_URI** to the appropriate one.

## How to Use in Shell

To apply these variables to your current shell session, use:

```bash
. ./key/-env.sh
```

This command sources the script, making the variables available in your current session. Ensure `-env.sh` is in the `key` directory relative to your current location.