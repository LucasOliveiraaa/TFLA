# TFLA CC 
TFLA CC means TFLA Configuration "Compiler", it's to convert a TFLAC file to a list of Searchers and Assemblers to the TFLA Algorithm.

## Installation
You can install TFLA CC using the installation tool, you can do that using one of those commands 
<br>
curl: `sh -c "$(curl -fsSL https://raw.githubusercontent.com/LucasOliveiraaa/TFLA/main/src/tfla_cc/tools/install.sh)"`
wget: `sh -c "$(wget -O- https://raw.githubusercontent.com/LucasOliveiraaa/TFLA/main/src/tfla_cc/tools/install.sh)"`
fetch: `sh -c "$(fetch -o- https://raw.githubusercontent.com/LucasOliveiraaa/TFLA/main/src/tfla_cc/tools/install.sh)"`

## Help
TFLA CC - TFLA Config \"Compiler\"<br>
<br>
tfla-cc \<command> [arguments]<br>
<br>
General Commands:<br>
&emsp;--help | -h <br>
&emsp;&emsp;Arguments: [command]<br>
&emsp;&emsp;Show this message or show the help of some command.<br>
&emsp;--digest | -d<br>
&emsp;&emsp;Arguments: \<path> [-a] [-w|-W] [output]<br>
&emsp;&emsp;Compile a TFLAC source and show the result.<br>
&emsp;--analyse | -a <br>
&emsp;&emsp;Arguments: \<path><br>
&emsp;&emsp;Just analyse the source code to syntax and impossible cases.<br>
&emsp;--compile | -c <br>
&emsp;&emsp;Arguments: \<path><br>
&emsp;&emsp;Compile the passed source code with type .tflac to an .exaust.tfla file with same name.<br>
&emsp;&emsp;aka tfla-cc -d \<path> -a -W<br>
<br>
--digest | -d <br>
&emsp;-a - Analyse the TFLAC file.<br>
&emsp;-w \<output> - Write the result in the passed output file.<br>
&emsp;-W - Write the result in a .exauts.tfla file with same name of inut file.<br>

# TFLAC
The TFLAC (TFLA Configuration) language defines a set of rules for the recognition and construction of tokens and abstract syntax trees (ASTs). Let's explore the details of this language.

## Basic Structure
A TFLAC file is divided into lines, each representing an Arbitrary Block (AB), which serves as a unit for search or construction, depending on its content. Each AB is unique and indivisible, meaning one AB cannot invade another. If an AB starts with '--', it will be ignored:
```tflac
-- This is called a "comment"!
-- They are ignored by TFLA CC

-- Use comments to organize your code
```
As mentioned earlier, an AB containing a comment cannot invade an AB of a different type:
```tflac
-- This is accepted

<rule> : arguments -- This is not accepted; the comment AB is
                    invading another AB
```

## Types of ABs
There are two types of rules in an AB. The first is the Searcher, which looks for a token in the code, and the other is the Assembler, which looks for a sequence of tokens to create an AST.

### Searcher
```tflac
[my_searcher]  :  ^regex
^^^^^^^^^^^^^     ^^^^^^
Name              Arbitrary
```
The name of the Searcher is enclosed in brackets, indicating that it is a Searcher. The arbitrary part contains a regular expression that looks for an occurrence of the token. It is recommended to start with '^' to avoid multiple occurrences.

### Assembler
The syntax of an Assembler is similar to a Searcher, but its name, previously enclosed in brackets, is now enclosed in angle brackets:

```tflac
                           Ref. to a Searcher
                        A literal           |
   Ref. to an Assembler         |           |
                      ↓         ↓           ↓
<my_assembler>  :  <my> arguments [arbitrary]
^^^^^^^^^^^^^^     ^^^^^^^^^^^^^^^^^^^^^^^^^^
Name               Arbitrary Arguments
```
The Assembler accepts various arbitrary arguments. The arguments can reference other Assemblers, Searchers, or literals. With these arguments, TFLA can find patterns in the source code and create an AST. The order of Assemblers may, depending on the situation, influence the analysis.

If it is necessary to add different options for an Assembler, such as alternative possibilities, create a new rule with the same name with the other option:

```tflac
<my_assembler>  :  first possibility
<my_assembler>  :  second possibility
```

You can remove the rule name and use "|" instead of ":":
```tflac
<my_assembler>  :  first possibility
                :  second possibility
                |  third possibility
```

If an Assembler can always be optional, use the symbol ε as an option:
```tflac
<my_assembler>  :  first possibility
<my_assembler>  :  second possibility
<my_assembler>  :  ε
```
Here, the Assembler \<my_assembler> can always be optional, as one of its options contains the symbol ε.

#### Symbols
1. `(arguments)`: Create a group of arbitrary arguments.
2. `(... | ...)`: Create a choise of two or more possibilities of group of arbitrary arguments.
3. `(arguments)?`: Indicates that the argument sequence is optional.
4. `(arguments)...`: Indicates that the argument sequence can repeat indefinitely.
5. `(arguments)+`: Indicates that the argument sequence can repeat 1 or more times.
6. `(arguments)N`: Indicates that the argument sequence can repeat N times.
7. `(arguments)"V"...`: Indicates that the argument sequence can repeat indefinitely, but must be separated by V in quotes.
8. `(arguments)"V"+`: Indicates that the argument sequence can repeat 1 or more times, but must be separated by V in quotes.
9. `(arguments)"V"N`: Indicates that the argument sequence can repeat N times, but must be separated by V in quotes.

## Predefined Symbols
TFLAC contains pre-defined symbols to facilitate the writing of grammar rules. A pre-defined symbol is enclosed in colons. Here are some examples:
1. `:nwl:`: Represents a newline.
2. `:tab:`: Represents a tabulation.
3. `:eof:`: Represents an EOF (end of file).
4. `:eol:`: Represents an EOL (end of line).
5. `:noh:`: Represents a null character (\0).
6. `:num:`: Represents a number.

TFLA CC latest implement a way to users to create your own symbols using the in-line regex, the sintax of an in-line regex is simple, the name of the custom symbol must be enclosed by colons, the atribuition symbol is colon-r(:r) or just a colon(:) and the arbitrary is a regular expression. Here is an example:
```tflac
:identify_new_line: :r (\r)?\n
```

To use your custom symbol, you need to enclose the symbol name with colons:
```tflac
<my_rule> : :identify_new_line:
```

The reason for using Custom Symbols with in-line regex instead of Searchers is as follows: TFLA CC does not interpret Custom Symbols as Searchers. Therefore, a Custom Symbol does not interfere with the TFLA Algorithm when it begins tokenization because the regex in a Custom Symbol are not used to match tokens.

## Impossible Cases
Impossible cases are situations not accepted by TFLA CC, ranging from syntax problems in TFLAC code to ambiguities and infinite recursions. Some examples include:

1. Direct Infinite Recursion:

```tflac
<a> : <a>
```
In this case, the rule \<a> refers directly to itself, creating infinite recursion.

2. Indirect Infinite Recursion:

```tflac
<a> : <b>
<b> : <a>
```
Here, \<a> refers to \<b>, and \<b> refers to \<a>, creating an indirect circular dependency.

3. Ambiguity:

```tflac
<expression> : <term> "+" <expression>
<expression> : <term> "*" <expression>
<expression> : <term>
```
The grammar for mathematical expressions is ambiguous, as an expression like 2 * 3 + 4 can be interpreted in two ways.

4. Non-deterministic Reduction:

```tflac
<expression> : <term> "+" <factor>
<expression> : <term> "*" <factor>
<expression> : <term>
<factor>    : <number>
<factor>    : "(" <expression> ")"
```
During syntactic analysis, there may be ambiguity in choosing between reductions.

5. Non-LR(1) Grammar:

```tflac
<S> : "if" <condition> "then" <statement>
<S> : "if" <condition> "then" <statement> "else" <statement>
```
This grammar is not LR(1) due to the shift-reduce conflict associated with the keywords "then" and "else" in a conditional structure.

6. Inconsistency:

```tflac
<statement> : "if" <condition> "then" <statement>
<statement> : "if" <condition> "then" <statement> "else" <statement>
<statement> : "while" <condition> "do" <statement>
```
There is inconsistency in the rules, as the if-then structure may or may not have an else block, while the while structure has no corresponding option.

7. Undesired Expressiveness:

```tflac
<expression> : <term> "+" <expression>
<expression> : <term> "*" <expression>
<expression> : <term> "^" <expression>
<expression> : <term>
```
The grammar allows exponentiation (^), making it more complex than desired for a simple language.

-----------

Enjoy ❤️ 
