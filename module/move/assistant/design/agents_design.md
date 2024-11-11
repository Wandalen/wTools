# Agents

## Descirption

This file depicts a general framework for building **multi-agent AI systems**.

Main description of such system is writen using **YAML**.

The system consists of **nodes** and **edges**.

Currently, we assume that the only information type passed between nodes is **single text** and nodes are connected in one way like a **pipeline**.

In future, we can support other types like *list of strings*. *booleans*; we can deliver input to several nodes *in parallel*, etc.

## Nodes

Each node has an `id` property (its name).

### Input nodes

These nodes read an input from external environment and pass it to the graph.

Current **types**:

- Stding input node.
- File input node.

#### Stdin input node

**Description**: retrieves input from user from the command line. A prompt can be supplied additionally.

**Parameters**:

- `prompt`: Text to show before the input, e.g.: `Query: `, `Your question: `, etc.

#### File input node

**Description**: reads file from the `path` parameters and passes to the next node.

**Parameters**:

- `path`: path to the file to read.

### Processing nodes

Those nodes perform intermediate processing of information. It can be either a mechanical one using an external program, or a real LLM agent.

#### Script node

**Description**: takes input from the node, runs the specified program, passes the input to the program's stdin, reads the programs output and passes it to the next node (this refs UNIX philosophy).

**Paramters**:

- `path`: path to the executable.

#### Agent node

**Description**: the core node type, represents an LLM agent that transforms text in one from to another.

**Parameters**:

- `provider`: LLM provider e.g.: `openai`, `anthropic`, etc.
- `model`: LLM model name, e.g.: `gpt-4o-mini`, `claude`, etc.
- `system_message`: system message template.
- `user_message`: user message template.

`system_message` and `user_message` are templates that have a variable called `{input}`.

### Output nodes

These nodes take an input from other node and present it to the external world.

Current **types**:

- Stdout output node.
- File output node.

#### Stdout output node

**Description**: prints the input to the console.

**Parameters**:

- `prefix`: print text before the output, e.g.: `Answer: `, etc.

#### File output node

**Description**: saves input to a file.

**Parameters**:

- `path`: path to save the input.

## YAML description structure

Please refer to `examples/` directory.
