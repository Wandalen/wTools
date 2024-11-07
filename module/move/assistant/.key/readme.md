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

## How to Use with Docker

To use these environment variables with Docker, you can pass them to your Docker container using the `--env-file` option. Create a `.env` file with the same content as your `.key/-env.sh` and use it when running your Docker container:

```bash
docker run --env-file ./.env your-docker-image
```

This command will start a Docker container with the environment variables defined in your `.env` file.

## How to Use with Docker Compose

To use these environment variables with Docker Compose, you can specify the `.env` file directly when running the `docker-compose` command by using the `--env-file` option:

```bash
docker-compose --env-file ./.env up
```

This command will apply the environment variables from the `.env` file to all services defined in your `docker-compose.yml` file when you run `docker-compose up`.

Ensure that your `.env` file is correctly formatted and accessible to Docker and Docker Compose commands.