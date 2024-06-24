# EADHW
Homework repository for the position of embedded application developer.

The specification asked for a "safe" rust API interfacing with a basic C shared library, and no further assumptions were given.
Given the embedded nature of the target platforms, this repository assumes that the C shared library makes use of HAL calls or
other "very low level" features of the platform to perform the arithmetic operations desired.
Naturally, the code itself _here_ are the direct operations found in ANSI C, but the idea is that their code could be
easily swapped for said HAL calls/Machine instructions.

Another problem of knowing little to nothing about the target platform is that word/integer size is left open.
This is vital information to make sure that the C shared library and the consuming Rust library have the same ABI.
For the purposes of this homework, it was assumed a default integer size of 32.

The safety done at the rust level is also very small due to the lack of assumptions: it simply tries to take care of arithmetic errors.
For example, overflow or underflow errors when summing two big integers.
Another consequence of the lack of assumptions about the platform is that checking for multiplication overflow or underflow cannot be safely
done efficiently.
This is because the exact number of bits for integers is unknown so that e.g. a carry bit could be propagated for fast overflow detection.
In any case, these could be optimized later without jeopardizing the shared library and its Rust API as presented in this repo.

Finally, there is basically no application developed as the focus is the library.
The main rust executable is a simple CLI program that takes two numbers and performs one of the desired operations on it.
Just try running executable (or with `-h`) and you will get a prompt telling you how to use it.