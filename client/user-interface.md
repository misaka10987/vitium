# Client User Interface
@#$%^&`\
## Reserved Characters
### Character `\`
Character `\` is used as an escape character.
### Character `@`
Character `@` is used for out-game chat with/without target specified. Used as `<chat-message>@[target]` with `target` splitted by comma. For example:
> \>Hello, world!@Alice, Bob
>
This sends an in-game chat message `Hello, world!` to `Alice` and `Bob`.
### Character `#`
Character `#` is used for issuing server command. Usually this is used by the host. For example:
> \>#help
>
See also: [Command List]()
### Character `$`
Character `$` is used for using already-defined variables. For example:
> \>Wield the $w.
>
This is equivalant to
> \>Wield the pistol.
>
when `$w` is `pistol`.
### Character `%`
Character `%` is used for calling functions, with parameters splitted by `|`. For example:
> \>%mv Outside the house.  
> \>%s | $p1 | $p2 | Hello, world! 
>
`mv` is a pre-defined function which indicates movement of the player character. `s` says a chat message in-game to specified targets, with the last parameter as the content. Note that you can **ONLY** call one function in a turn.  
See also: [Function usage and pre-defined functions]()
### Character `^`
Currently `^` is used for client debugging commands, reserved by client implementation.
### Character `&`
Currently noe used.