# How to Test

This is a short description on how to test RLisp with various Commands

1. Lisp in Lisp - Running McCarthys Root of Lisp Interpreter

2. Other tests focused on all different capabilities of RLisp

to start, run the RLisp interpreter! To see how this works, please consider the README.


## Testing Root of Lisp

Since RLisp combines elements from ROL with other LISP dialects I needed to adapt the original code of the
Root of Lisp Interpreter in order fo it to run on RLisp.

The adapted definition of the Root of Lisp Interpreter can be found in the file: ROL_adapted.txt

When you start RLisp, type the following Command to load the definition into your environment: (load 'ROL_adapted.txt')

**Now you can proceed with the Testing:**

You can either type in the commands manually or directly load the file ROL_tests.test, where I prepared some expressions
to test with. To load this file, enter the command: (load 'ROL_tests.test')

## Testing other aspects of RLisp

To test all other aspects and expressions of RLisp I prepared another file, that contains a range of tests, I think will
give a nice overview.

As with Root of Lisp, you can either type in the commands manually or load the prepared test-file with:
(load 'broad_testing.test')