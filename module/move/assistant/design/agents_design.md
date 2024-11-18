# Agents

## Descirption

This file depicts a general framework for building **multi-agent AI systems**.

Main description of such system is writen using **YAML**.

The system consists of **nodes** and **edges**.

Currently, we assume that the only information type passed between nodes is **single text** and nodes are connected as a direct acyclic graph (DAC). Node can take input from other nodes, but it can generate only 1 output.

In future, we can support other types like *list of strings*. *booleans*; we can deliver input to several nodes *in parallel*, etc.

## YAML description structure

Please refer to `examples/` directory.

## Paths

In several places in YAML file there are values of **paths**. Paths resemble paths in a real file system, parts are delimited with `::`. Absolute path starts from `::`.\

Examples of paths:

- `output`: relative path to single element `output`.
- `event::stdout`: relative path to `stdout` through `event`.
- `::trigger::stdin`: absolute path to `stdin` through `trigger`.

Places where paths are used:

- In nodes - `type`: type of the node. Different types of nodes live in different dirs.
- In nodes - `next`: to which node pass the execution. Nodes live in `::nodes` dir.
- In nodes - `agent_reuse`: reuse conversation history of previous agent.
- In templates - `{{...}}`: take output from the node. Output of nodes live in `::output` dir.

All paths (expect absolute) **are subject to absolutization**. This means that every relative path will be implicitly turned out to absolute path. In case of any ambiguities an error will be thrown. Absolutization also depends on the context: in `next` fields paths are absolutized to `::nodes` dir, in templates - to `::output` and so on.

## Execution

YAML file contains section about `nodes:`. You can think of them as statements in a programming language. Next statement is encoded in `next:` field. Output of the nodes are stored in `::output` dir.

## Scenarios referencing

There are two builtin scenarios:

- `::scenario::entry`
- `::scenario::termination`

## Detailed description of nodes

Each node has an `id` property (its name) and a `type` property.

`type` specifies node type in a special format - `category::type`. Number of levels separated by dots may vary.

### Input nodes

These nodes read an input from external environment and pass it to the graph.

Current **types**:

- `trigger::stdin`: stdin input node.
  Parameters: `prompt`.
- `trigger::file`: file input node. 
  Parameters `path`.

### Processing nodes

Those nodes perform intermediate processing of information. It can be either a mechanical one using an external program, or a real LLM agent.

- `script`: script node.
  Parameters: `cmd`.
- `agent::completion`: agent completion node.
  Parameters: `provider`, `model`, `system_message`, `user_message`, `agent_reuse`.
  `system_message` and `user_message` are templates. Variables available to those templates are **node names**.

### Output nodes

These nodes take an input from other node and present it to the external world.

Current **types**:

- `event::stdout`: stdout output node.
  Parameters: `output`.
- `event::file`: file output node.
  Parameters: `path`.

### Utility nodes

These nodes are special nodes for various purposes.

Current **types**:

- `scenario::termination`: scenario termination node.
