grammar ProjectE;

program        : (declaration)* EOF ;

declaration    : classDecl
               | funDecl
               | varDecl
               | statement ;
               
varDecl        : 'var' typedVar ( '=' expression )? ';' ;
typedVar       : IDENTIFIER ':' type ;
type           : IDENTIFIER ( '.' IDENTIFIER )* ;

funDecl        : 'fun' IDENTIFIER? function ;
function       : '(' parameters? ')' ( ':' type )? block ;
parameters     : typedVar ( ',' typedVar )* ;

classDecl      : 'class' IDENTIFIER ( 'extends' type )?
                 '{' ( method | field )* '}' ;
method         : IDENTIFIER function ;
field          : typedVar ( '=' expression )? ';' ;

statement      : exprStmt
               | forStmt
               | ifStmt
               | returnStmt
               | whileStmt
               | breakStmt
               | continueStmt
               | includeStmt
               | block ;

includeStmt    : 'include' STRING ';' ;

returnStmt     : 'return' expression? ';' ;

forStmt        : 'for' '(' ( varDecl | exprStmt | ';' )
                 expression? ';'
                 expression? ')' statement ;

whileStmt      : 'while' '(' expression ')' statement ;

breakStmt      : 'break' ';' ;

continueStmt   : 'continue' ';' ;

ifStmt         : 'if' '(' expression ')' statement
               ( 'else' statement )? ;

block          : '{' declaration* '}' ;

exprStmt       : expression ';' ;

expression     : assignment ;
assignment     : ( call '.' )? IDENTIFIER '=' assignment
               | logic_or ;
logic_or       : logic_and ( 'or' logic_and )* ;
logic_and      : equality ( 'and' equality )* ;
equality       : comparison ( ( '!=' | '==' ) comparison )* ;
comparison     : term ( ( '>' | '>=' | '<' | '<=' ) term )* ;
term           : factor ( ( '-' | '+' ) factor )* ;
factor         : unary ( ( '/' | '*' ) unary )* ;
unary          : ( '!' | '-' ) unary | update ;
update         : call ( '++' | '--' )? | ( '++' | '--' ) call ;
arguments      : expression ( ',' expression )* ;
call           : primary ( '(' arguments? ')' | '.' IDENTIFIER
               | '[' expression ']' )* ;
primary        : 'true' | 'false' | 'null' | 'this' | 'super'
               | NUMBER | STRING | IDENTIFIER | '(' expression ')'
               | 'fun' function ;

NUMBER: DIGIT ('.' DIGIT)?;
STRING: ('"' (~('"'))*  '"');

IDENTIFIER: (ALPHA (ALPHA|NUMBER)*);

ALPHA: ('a' .. 'z' | 'A' .. 'Z' | '_')+;
DIGIT: ('0' .. '9')+;

NEWLINE: ('\n' | '\r')+ -> channel(HIDDEN);
COMMENT: (('#' | '//') ~('\r' | '\n')*) -> channel(HIDDEN);
MULTILINE_COMMENT: '/*' .*? '*/' -> channel(HIDDEN);
WHITESPACE: (' ' | '\t')+ -> channel(HIDDEN);


