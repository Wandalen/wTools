# Commands

## Legend

- `<...>` - argument.
- `<..?>` - optional argument.
- `<...=...>` - argument with default value.
- `(...)+` - one or more times.

## OpenAI

### Files

```shell
assistant openai files upload <path> <purpose>
assistant openai files list
assistant openai files retrieve <id>
assistant openai files delete <id>
assistant openai files retrieve-content <id>
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

