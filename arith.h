/* This header expores basic arithmetic operations from the target platform.
   The operations are provided in an "unsafe" manner,
   i.e. arithmetic errors like underflow and overflow might happen in any call.
   The intended use for this header is through the rust crate "arith" which
   smartly wraps these calls in safe manner.
 */

/* Adds two signed integer using platform-specific operations of default size.
   Refer to the platform's specification to known the default integer size. */
int add(int a, int b);

/* Subtracts two signed integer using platform-specific operations of default size.
   Refer to the platform's specification to known the default integer size. */
int subtract(int a, int b);

/* Multiplies two signed integer using platform-specific operations of default size.
   Refer to the platform's specification to known the default integer size. */
int multiply(int a, int b);

/* Divides two signed integer using platform-specific operations of default size
   and returns a IEEE754 double-precision floating point number.
*/
double divide(int numerator, int denominator);