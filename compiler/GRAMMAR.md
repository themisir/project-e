# Grammar

```
program        → declaration* EOF ;

declaration    → classDecl
               | funDecl
               | varDecl
               | statement ;

classDecl      → "class" IDENTIFIER ( "<" IDENTIFIER )?
                 "{" function* "}" ;

funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" block ;
parameters     → IDENTIFIER ( "," IDENTIFIER )* ;

statement      → exprStmt
               | forStmt
               | ifStmt
               | printStmt
               | returnStmt
               | whileStmt
               | breakStmt
               | continueStmt
               | includeStmt
               | block ;

includeStmt    → "include" STRING ";" ;

returnStmt     → "return" expression? ";" ;

forStmt        → "for" "(" ( varDecl | exprStmt | ";" )
                 expression? ";"
                 expression? ")" statement ;

whileStmt      → "while" "(" expression ")" statement ;

ifStmt         → "if" "(" expression ")" statement
               ( "else" statement )? ;

block          → "{" declaration* "}" ;

exprStmt       → expression ";" ;

printStmt      → "print" expression ";" ;

varDecl        → "var" IDENTIFIER ( "=" expression )? ";" ;

expression     → assignment ;
assignment     → ( call "." )? IDENTIFIER "=" assignment
               | logic_or ;
logic_or       → logic_and ( "or" logic_and )* ;
logic_and      → equality ( "and" equality )* ;
equality       → comparison ( ( "!=" | "==" ) comparison )* ;
comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
term           → factor ( ( "-" | "+" ) factor )* ;
factor         → unary ( ( "/" | "*" ) unary )* ;
unary          → ( "!" | "-" ) unary | update ;
update         → call ( "++" | "--" )? | ( "++" | "--" ) call ;
arguments      → expression ( "," expression )* ;
call           → primary ( "(" arguments? ")" | "." IDENTIFIER
               | "[" expression "]" )* ;
primary        → "true" | "false" | "null" | "this" | "super"
               | NUMBER | STRING | IDENTIFIER | "(" expression ")" ;
```