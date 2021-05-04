# RLisp
A Metacircular LISP Interpreter written in Rust

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

### As Commandline Interpreter

When just executing Rlisp without any arguments, you start the Commandline Interpreter (similar to executing python3).

'''
Rlisp   (Windows)
./Rlisp (Unix)
'''

Then you can enter your code expression for expression, line for line.
