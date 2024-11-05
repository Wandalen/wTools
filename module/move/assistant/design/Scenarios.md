# Scenarios

## OpenAI

### Projects

#### Create and add users to a new project

```shell
assistant openai projects create 'AnotherAI'
```

Administrator will receive the project ID (referred as `<project_id>`).

Then, to add users to the projects, administrator should know their IDs (referred as `<user1_id>` and `<user2_id>`).

```shell
assistant openai project-users create <project_id> <user1_id> owner
assistant openai project-users create <project_id> <user2_id> member
```

Now, to check that the list of users is filled correctly, one can use:

```shell
assistant openai project-users list <project_id>
```

To list project API keys, one can use:

```shell
assistant openai project-api-keys list <project_id>
```

#### Project "rebranding"

Consider situation:

1. Owner of the organization is changed.
2. New owner decided to rename the project.
3. Also new owner decided to delete and add new employees to the project.

Changing the organization owner is done like this:

```shell
assistant openai users modify <new_onwer_id> owner
assistant openai users modify <old_owner_id> reader
```

To rename the project and reassign employees, these commands are used:

```shell
assistant openai projects modify <project_id> 'AnotherAwesomeAI'
assistant openai project-users delete <project_id> <old_employee_id>
assistant openai project-users create <project_id> <new_employee_id> member
```


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
