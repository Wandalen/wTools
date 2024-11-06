# Commands

## Legend

- `<...>` - argument.
- `<..?>` - optional argument.
- `<...=...>` - argument with default value.
- `(...)+` - one or more times.

## OpenAI

### Users

```shell
assistant openai users list
assistant openai users modify <id> <role>
assistant openai users retrieve <id>
assistant openai users delete <id>
```

### Projects

```shell
assistant openai projects list <include_archived=false>
assistant openai projects create <name>
assistant openai projects retrieve <id>
assistant openai projects modify <id> <new_name>
assistant openai projects archive <id>
```

### Project users

```shell
assistant openai project-users list <project_id>
assistant openai project-users create <project_id> <user_id> <role>
assistant openai project-users retrieve <project_id> <user_id>
assistant openai project-users modify <project_id> <user_id> <role>
assistant openai project-users delete <project_id> <user_id>
```

### Project API keys

```shell
assistant openai project-api-keys list <project_id>
assistant openai project-api-keys retrieve <project_id> <key_id>
assistant openai project-api-keys delete <project_id> <key_id>
```

### Assistants

```shell
assistant openai assistants create <model> <name?> <description?> <instructions?>
assistant openai assistants list
assistant openai assistants retrieve <id>
assistant openai assistants modify <id> <model?> <name?> <description?> <instructions?>
assistant openai assistants delete <id>
```

### Threads

```shell
assistant openai threads create
assistant openai threads retrieve <id>
assistant openai threads delete <id>
```

### Messages

```shell
assistant openai messages create <thread_id> <role> <content>
assistant openai messages list <thread_id>
assistant openai messages retrieve <thread_id> <message_id>
assistant openai messages modify <thread_id> <message_id>
assistant openai messages delete <thread_id> <message_id>
```

### Chat

```shell
assistant openai chat create-completion (<role> <message>)+
```

### Runs

```shell
assistant openai runs create <thread_id> <assistant_id>
assistant openai runs create-with-thread <assistant_id> <user_message>
assistant openai runs list <thread_id>
assistant openai runs retrieve <thread_id> <run_id>
assistant openai runs cancel <thread_id> <run_id>
```

## Anthropic

### Messages

```shell
assistant anthropic messages create (<role> <message>)+
```

