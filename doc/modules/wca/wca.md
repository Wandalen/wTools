# Diagrams

## Class diagram

- `Parser`
> Parse raw string to `RawCommand`-s

- `Grammar`
> Available commands configured by the user.
>
> Converts `RawCommand`-s to `GrammarCommand`-s with validation.
>
> `GrammarComand` contains valid subject and property values

- `Executor`
> Available routines configured by the user.
>
> Converts `GrammarCommand`-s to `ExecutableCommand`-s
>
> This entity is responsible for the program execution process

- `CommandsAggregator`
> Configures `Parser`, `Grammar` and `Executor` by the user.
>
> Executes the entire pipeline (parse -> validate -> execute)

<div style="background-color: #FFFFFF; padding: 10px; border-radius: 8px;">
    <img src="https://i.imgur.com/uW70tQg.png" title="Class diagram" />
</div>

## Sequence diagram

<div style="background-color: #FFFFFF; padding: 10px; border-radius: 8px;">
    <img src="https://i.imgur.com/LmUP7QK.png" title="Sequence diagram" />
</div>
