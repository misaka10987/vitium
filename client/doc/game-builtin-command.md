# Game builtin commands
- `alias` is used just like bash
- Client should support tagging raw UIDs with user-defined names and use `$` to invoke them
- `'$text` for NL.

|Command|Args|Meaning|
|-|-|-|
|`say`|`$message-text`|Says something to all people in the current scenario|
||`$message-text $dest`|Says something to specified people|
|`wield`|`$item-uid`|Wields specified item. By default, this tries to put the current item on hand into inventory, and drops it if unable|
||None|Shows information of wielding item|
||`-w`|Stop wielding current item and put it into inventory|
|`drop`|`$[item-uid]`|Drops the specified item(s) on the ground|
||`--all`|Drops all items on the ground|
|`pick`|`$[item-uid]`|Picks up specified item(s)|
|`mv`|`$place-uid`|Moves to the specified place|
|`attack`|`$target [$[fallback-targets]]`|Launch a melee attack to specified target with current item|
|`shoot`|`$target [$[fallback-targets]]`|Shoot specified target with current item|
|`cast`|`$spell $target $[fallback-targets]`|Casts selected spell to target(s)|
|`wear`|`$[item-uid]`|Wears specified armor(s)|
||None|Show wearing armors|
|`takeoff`|`$[item-uid]`|Takes off specified armor(s)|
||`--all`|Takes off all armor|
|`relax`|`$minute`|Relax for specified time|
|`sleep`|`$hour`|Sleep for specified time|
|`activate`|`$uid.$feature`|Activates a feature|
