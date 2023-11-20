# PoPL Project G2

Instructions for compilation and running
-----------------------------------------
Cargo:

cargo build 

cargo run < word to search for> <file in which to search > [ any character for case sensitive/not sensitive ]

C:

gcc grep_impli.c

./a.out [-c for case sensitivity ] < word to search for> <file in which to search >


Imperitive vs Functional(Declarative programming language)
----------------------------------------------------------
Imperitive programming languages like C describes a sequence of steps(algorithms) followed to reach a particular result. The focus here is on the 'How?'.
Functional programming languages like Rust are more focused on 'What?' is to be obtained rather than how it is obtained.

There is a certain level of abstraction that is associated with functional programming. This is evident in the usage of functions like map() and filter() in this code. For someone new to the language, such functions lack clarity and lead to additional cognitive load. However once the programmer gets used to the language, these fuctionalities prove to be highly efficient and cut down on many lines of code that may be required to carry out the same operation in an imperitive programming language.

Declarative programming really shines when the user is only concerned with the “what” and not the “how”. This makes sense in cases like when designing an API layer on top of a more complex framework.

The reality is that somewhere underneath any declarative system there will be imperative programming driving it.

The advantage of utilising Rust here is the additional memory safety. Rust employs a unique ownership system where each value in Rust has a variable that is its "owner." There are strict rules governing ownership, borrowing, and references that prevent common issues like null pointer dereferencing, dangling pointers, and memory leaks. In C, it is easy for the programmer's mistakes like not freeing allocated memory to cause differences in the memory utilisation.

Results
---------
40 lines  – Rust

Result - more time but lesser CPU utilisation
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/63d7aec2-7802-45e6-8be1-6b4bfc1464c0)

40 lines  – C 

Result - less time but more CPU utilization
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/92e23985-a34f-49a8-9862-dd9c9d9f40e3)

500,000 lines - Rust 

Result - more time but lesser CPU utilisation
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/12a166d9-299b-4cd8-90c1-8f8dc985b0c7)



500,000 lines – C 

Result - less time but more CPU utilization
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/c2107eff-fe3a-4cd1-bca2-4b54711a73b5)


1 million+ lines - Rust 

Result - more time but lesser CPU utilisation
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/0a3e2e13-e98c-452d-9ed0-22f54b531f3e)


1 million+ lines – C 

Result - less time but more CPU utilization
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/92772716/2b4e13fb-0988-4b1f-b217-a622cc176d2a)

Results for Run Time memory consumed
------
Rust
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/5864dbe7-dedc-44c8-977e-e978b239f9c5)


C
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/dcb22fad-b80d-44b8-a709-f57383c96f2b)


Tabulated Results For Time
---------------------------
![Screenshot from 2023-11-17 15-55-18](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/abf46738-333e-4f48-9b28-71d877c2e3d7)
![Screenshot from 2023-11-17 15-56-46](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/f2f34e89-6b36-42cf-a69c-eff3b77df50a)

Difference in data when failing to deallocate memory in C - this is not possible to do in Rust due to how the langauge is designed
----
Both non deallocated and definitely leaked memory is seen.
![image](https://github.com/shreya-aiyer/PoPL_Project_G2/assets/93695659/b1ef3aba-db81-4e1b-a6a6-165c40742fb6)


References
----------
Crate used for rich text (colours for found words and errors) - https://docs.rs/r3bl_rs_utils/latest/r3bl_rs_utils/

Resources for learning Rust - https://doc.rust-lang.org/book/

impl keyword - https://doc.rust-lang.org/std/keyword.impl.html

Accepting command line arguments in Rust and minigrep - https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html#:~:text=Reading%20the%20Argument%20Values,line%20arguments%20passed%20to%20minigrep%20.

Grep source code - https://github.com/git/git/blob/master/grep.c
