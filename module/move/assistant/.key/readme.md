# Keys

This document provides a concise example of an environment configuration script, used to set up environment variables for a project. These variables configure application behavior without altering the code.

## Example of `.key/-env.sh`

```bash
# OpenAI API key.
OPENAI_API_KEY=sk-proj-ABCDEFG
```

## How to Use in Shell

To apply these variables to your current shell session, use:

```bash
. ./key/-env.sh
```

This command sources the script, making the variables available in your current session. Ensure `-env.sh` is in the `key` directory relative to your current location.