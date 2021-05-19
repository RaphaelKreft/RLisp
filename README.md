# RLisp

A Metacircular LISP Interpreter written in Rust. The goal of this Interpreter is to be able to execute a universal fucntion, in my case
the RootOfLisp Interpreter from ... . This makes RLisp metacircular -> Running a LISP Interpreter in RLisp.
In Addition to that the Interpreter should also have some more functionality such as Integer arithmetics.

For a Full Documentation of the Language see section "Documentation".

## Usage

The Interpreter can be used in 2 operating modes:

### Run a file

You can run the Interpreter in this mode to execute a whole file. Similar to the famous python Interpreter
where you do:

'''
python3 myfile.py
'''

You can use RLisp in similar way:

'''
RLisp myfile.file (Windows)
./Rlisp myfile.file (Linux)
'''

In this Version it just reads the file line by line, so make sure each expression is on one line.

### As Commandline Interpreter

When just executing Rlisp without any arguments, you start the Commandline Interpreter (similar to executing python3).

'''
Rlisp   (Windows)
./Rlisp (Unix)
'''

Then you can enter your code expression for expression, line for line.

## Documentation

## How It works


## Recources and building process

I built this Interpreter in a University Project at University Basel. It was a lot of fun but also challenging, since I never touched the programming Language Rust before. With this said: please don't expect very clever Rust Code (I did my best but I learned the language along with the project.) and if you find something that could have been done a lot better or something completely wrong, I would appreciate if you raise an issue.

The main recource I used, was the MAL project, where I got the idea for the parser from and the well known book: 
