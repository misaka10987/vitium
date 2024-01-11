# Vitium built-in in-game functions guide
This document is made as a referance for using Vitium built-in functions. All functions included here should be implemented regardless of what mods are currently loaded.

---
## Function `a`
The function `a` indicates to **a**ctivate a certain skill to the target specified. Depending on the skill used, this is possible to be inflicted on multiple targets.
### Usage
```
%a <skill-id> | <target1> | [target2] ... [targetN]
```

---
## Function `fight`
The function `fight` indicates to start a battle with the specified target(s).
### Usage
```
%fight <target1> | [target2] ... [targetN]
```

---
## Function `mv`
The function `mv` indicates **m**o**v**ement of player character.
### Usage
```
%mv <dest>
```
or
```
%mv <shortcut>
```
### Shortcuts
|Shortcut|Meaning|
|-|-|
|`.`|Go forward for a few steps.|
|`-.`|Go backward for a few steps.|

---
## Function `s`
The function `s` sends an in-game chat message to specified characters.
### Usage
```
%s [ [chara1] | [chara2] | ... | [charaN] ] = $all | <message>
```
### Note
In some client implementations, this can also be used via `@`.

---
## Function `shoot`
The function `shoot` indicates to shoot a target. Target can be either player character or non-player character.
### Usage
```
%shoot <target>
```

---
## Function `w`
The function `w` indicates to **w**ield an item.
### Usage
```
%w <item-name>
```
### Note
If you have multiple items that share one name, the client should make notice and ask to specify one.