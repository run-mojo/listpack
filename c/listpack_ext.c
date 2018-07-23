//
//
//

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <ctype.h>
#include <limits.h>
#include <math.h>
#include <unistd.h>
#include <sys/time.h>
#include <float.h>
#include <stdint.h>
#include <errno.h>
#include "listpack.h"

#define LONG_STR_SIZE      21          /* Bytes needed for long -> str + '\0' */

// init with libc malloc
void* (*lp_malloc)(size_t) = malloc;
// init with libc realloc
void* (*lp_realloc)(void*,size_t) = realloc;
// init with libc free
void (*lp_free)(void*) = free;


/* Return the number of digits of 'v' when converted to string in radix 10.
 * See ll2string() for more information. */
uint32_t lp_digits10(uint64_t v) {
    if (v < 10) return 1;
    if (v < 100) return 2;
    if (v < 1000) return 3;
    if (v < 1000000000000UL) {
        if (v < 100000000UL) {
            if (v < 1000000) {
                if (v < 10000) return 4;
                return 5 + (v >= 100000);
            }
            return 7 + (v >= 10000000UL);
        }
        if (v < 10000000000UL) {
            return 9 + (v >= 1000000000UL);
        }
        return 11 + (v >= 100000000000UL);
    }
    return 12 + lp_digits10(v / 1000000000000UL);
}

/* Like digits10() but for signed values. */
uint32_t lp_sdigits10(int64_t v) {
    if (v < 0) {
        /* Abs value of LLONG_MIN requires special handling. */
        uint64_t uv = (v != LLONG_MIN) ?
                      (uint64_t)-v : ((uint64_t) LLONG_MAX)+1;
        return lp_digits10(uv)+1; /* +1 for the minus. */
    } else {
        return lp_digits10(v);
    }
}

/* Convert a long long into a string. Returns the number of
 * characters needed to represent the number.
 * If the buffer is not big enough to store the string, 0 is returned.
 *
 * Based on the following article (that apparently does not provide a
 * novel approach but only publicizes an already used technique):
 *
 * https://www.facebook.com/notes/facebook-engineering/three-optimization-tips-for-c/10151361643253920
 *
 * Modified in order to handle signed integers since the original code was
 * designed for unsigned integers. */
int lp_ll2string(char *dst, size_t dstlen, long long svalue) {
    static const char digits[201] =
            "0001020304050607080910111213141516171819"
            "2021222324252627282930313233343536373839"
            "4041424344454647484950515253545556575859"
            "6061626364656667686970717273747576777879"
            "8081828384858687888990919293949596979899";
    int negative;
    unsigned long long value;

    /* The main loop works with 64bit unsigned integers for simplicity, so
     * we convert the number here and remember if it is negative. */
    if (svalue < 0) {
        if (svalue != LLONG_MIN) {
            value = -svalue;
        } else {
            value = ((unsigned long long) LLONG_MAX)+1;
        }
        negative = 1;
    } else {
        value = svalue;
        negative = 0;
    }

    /* Check length. */
    uint32_t const length = lp_digits10(value)+negative;
    if (length >= dstlen) return 0;

    /* Null term. */
    uint32_t next = length;
    dst[next] = '\0';
    next--;
    while (value >= 100) {
        int const i = (value % 100) * 2;
        value /= 100;
        dst[next] = digits[i + 1];
        dst[next - 1] = digits[i];
        next -= 2;
    }

    /* Handle last 1-2 digits. */
    if (value < 10) {
        dst[next] = '0' + (uint32_t) value;
    } else {
        int i = (uint32_t) value * 2;
        dst[next] = digits[i + 1];
        dst[next - 1] = digits[i];
    }

    /* Add sign. */
    if (negative) dst[0] = '-';
    return length;
}

/* This is just a wrapper for lpAppend() to directly use a 64 bit integer
 * instead of a string. */
unsigned char *lpAppendInt64(unsigned char *lp, int64_t value) {
    char buf[LONG_STR_SIZE];
    int slen = lp_ll2string(buf,sizeof(buf),value);
    return lpAppend(lp,(unsigned char*)buf,slen);
}

unsigned char *lpInsertInt64(unsigned char *lp, int64_t value, unsigned char *p, int where, unsigned char **newp) {
    char buf[LONG_STR_SIZE];
    int slen = lp_ll2string(buf,sizeof(buf),value);
    return lpInsert(lp, (unsigned char*)buf, slen, p, where, newp);
}

/* This is just a wrapper for lpReplace() to directly use a 64 bit integer
 * instead of a string to replace the current element. The function returns
 * the new listpack as return value, and also updates the current cursor
 * by updating '*pos'. */
unsigned char *lpReplaceInt64(unsigned char *lp, unsigned char **pos, int64_t value) {
    char buf[LONG_STR_SIZE];
    int slen = lp_ll2string(buf,sizeof(buf),value);
    return lpInsert(lp, (unsigned char*)buf, slen, *pos, LP_REPLACE, pos);
}

/* This is a wrapper function for lpGet() to directly get an integer value
 * from the listpack (that may store numbers as a string), converting
 * the string if needed. */
//int64_t lpGetInteger(unsigned char *ele) {
//    int64_t v;
//    unsigned char *e = lpGet(ele,&v,NULL);
//    if (e == NULL) return v;
//    /* The following code path should never be used for how listpacks work:
//     * they should always be able to store an int64_t value in integer
//     * encoded form. However the implementation may change. */
//    long long ll;
//    int retval = string2ll((char*)e,v,&ll);
//    serverAssert(retval != 0);
//    v = ll;
//    return v;
//}

