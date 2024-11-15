# Agents

## Descirption

This file depicts a general framework for building **multi-agent AI systems**.

Main description of such system is writen using **YAML**.

The system consists of **nodes** and **edges**.

Currently, we assume that the only information type passed between nodes is **single text** and nodes are connected as a direct acyclic graph (DAC). Node can take input from several nodes, but it can generate only 1 output.

In future, we can support other types like *list of strings*. *booleans*; we can deliver input to several nodes *in parallel*, etc.

## Nodes

Each node has an `id` property (its name) and a `type` property.

`type` specifies node type in a special format - `category.type`. Number of levels separated by dots may vary.

### Input nodes

These nodes read an input from external environment and pass it to the graph.

Current **types**:

- `trigger.stdin`: stdin input node.
- `trigger.file`: file input node.

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

- `script`: script node.
- `agent.completion`: agent completion node.

#### Script node

**Description**: takes input from the node, runs the specified program, passes the input to the program's stdin, reads the programs output and passes it to the next node (this refs UNIX philosophy).

**Paramters**:

- `cmd`: command to execute with path to the executable and arguments.

#### Agent completion node

**Description**: the core node type, represents an LLM agent that transforms text in one from to another.

**Parameters**:

- `provider`: LLM provider e.g.: `openai`, `anthropic`, etc.
- `model`: LLM model name, e.g.: `gpt-4o-mini`, `claude`, etc.
- `system_message`: system message template.
- `user_message`: user message template.

`system_message` and `user_message` are templates. Variables available to those templates are **node names**.

### Output nodes

These nodes take an input from other node and present it to the external world.

Current **types**:

- `event.stdout`: stdout output node.
- `event.file`: file output node.

#### Stdout output node

**Description**: prints the input to the console.

**Parameters**:

- `prefix`: print text before the output, e.g.: `Answer: `, etc.

#### File output node

**Description**: saves input to a file.

**Parameters**:

- `path`: path to save the input.

### Utility nodes

These nodes are special nodes for various purposes.

Current **types**:

- `scenario.termination`: scenario termination node.

#### Scenario termination node

**Description**: when the process of execution is passed to this node, the whole program of the multi-agent system terminates.

This node is **implicitly present in every graph**, and to call it you just need to fill `next:` with the `scenario.terminate`.

## YAML description structure

Please refer to `examples/` directory.
