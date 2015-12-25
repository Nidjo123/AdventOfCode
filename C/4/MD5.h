#ifndef MD5_H
#define MD5_H

#include <string.h>
#include <stdlib.h>
#include <stdio.h>

#define DIGEST_LENGTH 16

char* MD5(char* msg, char* digest_p);
void hex_digest(char* digest);

#endif