This is a simple language designed to be easily compiled to any other language



## Types
A single integer type


## Syntax

```ebnf
program         =   { statement } .
statement       =   assignment
                |   loop_statement
                |   print_statement
assignment      =   variable_name "=" expression
expression      =   operand { operator operand }
operand         =   variable_name | integer
operator        =   "+" | "-"
loop_statement  =   "loop" operand "{" { statement } "}"
print_statement =   "print" operand
```


Example :
```
x = 2
y = 0
loop x {
    y = y + 2
}
print y
```