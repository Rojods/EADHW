/* This file implements the arithmetic operations declared in "arith.h" for the target platform.
   Please refer to "arith.h" for further documentation.
*/
#include "arith.h"

int add(int a, int b)
{
    return a + b;
}

int subtract(int a, int b)
{
    return a - b;
}

int multiply(int a, int b)
{
    return a * b;
}

double divide(int numerator, int denominator)
{
    return ((double)numerator) / ((double)denominator);
}