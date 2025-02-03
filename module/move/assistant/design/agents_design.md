# Agents

## YAML description structure

Please refer to `examples/` directory.

## Paths

- Used in node types, templates.
- Parts are delimited with `::`.
- Absolute path has a leading `::`.
- All paths (expect absolute) **are subject to absolutization**. Absolutization also depends on the context: in `next` fields paths are absolutized to `::nodes` dir, in templates - to `::output` and so on.

## Execution

- YAML file contains section about `nodes:`.
- Next node is encoded in `next:` field. 
- Output of the nodes are stored in `::output` dir.

## Builtin scenarios

- `::scenario::entry`
- `::scenario::termination`

## Node types

- Input nodes:
  - `trigger::stdin`
  - `trigger::file`
- Processing nodes:
  - `script`
  - `agent::completion`
- Output nodes:
  - `event::stdout`
  - `event::file`

Refer to examples in `examples/` to see fields of nodes.