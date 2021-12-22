# Grammar

```
program        → declaration* EOF ;

declaration    → classDecl
               | funDecl
               | varDecl
               | statement ;
               
varDecl        → "var" typedVar ( "=" expression )? ";" ;
typedVar       → IDENTIFIER ":" type ;
type           → IDENTIFIER ( "." IDENTIFIER )* ;

classDecl      → "class" IDENTIFIER ( "extends" type )?
                 "{" ( function | property )* "}" ;
field          → typedVar ";" ;

funDecl        → "fun" function ;
function       → IDENTIFIER "(" parameters? ")" ( ":" type )? block ;
parameters     → typedVar ( "," typedVar )* ;

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