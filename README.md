# m3th
m3th (my simple programming language) compiler implementation

## CLI
```
m3th build
m3th run
```

## File extension
`file.m3th`

## Language specification

### Entrypoint
```
fn main(): vacuum {
    // ...
}
```

### Comments
```
// Comment here
```

### Operator
```
+ - * / % ( ) < > <= >= == != ->
```

### Variables
To declare:
- Mutable variable: `mut a = 1`
- Immutable variable: `immut a = 1`

Declare with type:
`mut a: str = "Hello, World"`

### Types
```
int double bool str vacuum
```

### Expressions

#### Conditional
```
when <expr> {
    <expr>
}

when <expr1> {
    <expr>
} or when <expr2> {
    <expr>
} or {
    <expr>
}
```


#### Iterate
- `all`: apply same function to all items
- `any`: filter out items that match conditions
- `accum`: accumulate items

```
all(lst, fn(immut c) { c * 2; })
any(lst, fn(immut c) { c == 2; })
accum(lst, 0, fn(mut s, immut c) { s = s + c; })
```

### Functions
Arguments:
- `mut`: Mutable
- `mutref`: Mutable + Reference
- `immut`: Immutable

```
fn calculate(mut a: int, mutref b: bool, immut c: float): bool {
    -> true;
}
```

Lambda function:
- Arguments is immutable, and these types can be guessed.
- The return type can be guessed.
```
fn is_true(a) -> a == 1
```

### Packages

Declare package:

```
pkg main
```

Use other package
```
need "package_name_here" as als;
```

Change the visibility to public:
```
show a;
show fn calculate(): bool {
    -> true;
}
```

Access to other packages:
```
als.a
als.function()
```
