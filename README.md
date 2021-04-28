# roll - commandline pen&paper dice roller


##usage
just add the dice you want to roll as arguments.
mathematic operations and dice can be compined as you want.

###Example
```
./roll (2d6+3)*5
./roll 2d6+1d4+5d8
./roll 1d%
./roll 1d%*5+(21+1d6)+123d456
```

##Operators
Operator | Desc
--- | --- 
 `+` | addition
 `-` | subtraction
 `*`  | multipication
 `/` | division (rounded mathematically)
 `(   )` | braces for calculation order

##Dice
[n]d[m]-> n and m can be any numeric unsigned 64bit value greater then 0

also works with [n]D[m], [n]w[m] and [n]W[m]





