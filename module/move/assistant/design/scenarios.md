# Scenarios

## OpenAI

### Assistants

#### Make new assistant

```shell
assistant openai assistants create gpt-4o-mini CoolBot 'CoolBot is a helpful assistant.' 'You are a helpful assistant.'
```

This command will return assistant ID.

#### Chat with the assistant

To chat with OpenAI assistant, one should do this:

1. Create a thread. Thread is like a chat.
2. Write a message in thread (e.g. a question).
3. Run the assistant in the thread.

```shell
assistant openai threads create
```

This command will return the new thread ID (referred as `thread_id`). To call an assistant, you need to know its ID.

```shell
assistant openai messages create <thread_id> user '2 + 2 = ?'
assistant openai runs create <thread_id> <assistant_id>
```
