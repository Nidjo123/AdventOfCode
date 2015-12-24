#include <stdio.h>
#include <string.h>
#include "MD5.h"

#define MAXN 1000
#define ZEROS 5

int get10n(int n) {
	int res = 1;
	
	while (n) {
		n /= 10;
		res *= 10;
	}
	
	return res / 10;
}

void append(char* msg, int len, int num) {
	int i = len;
	int curr = get10n(num);
	
	while (num) {
		msg[i++] = '0' + num / curr;
		num %= curr;
		curr /= 10;
	}
	
	msg[i] = 0;
}

int check_digest1(char* digest) {
	return digest[0] == 0 && digest[1] == 0 && (digest[2] & 0xf0) == 0;
}

int check_digest2(char* digest) {
	return digest[0] == 0 && digest[1] == 0 && digest[2] == 0;
}

int main(void) {
	char msg[MAXN + 1];
	char digest[DIGEST_LENGTH];
	int len;
	int num = 0, done1 = 0, done2 = 0;
	
	scanf("%s", msg);

	len = strlen(msg);
	
	while (!done1 || !done2) {
		num++;
		append(msg, len, num);
		
		MD5(msg, digest);
		
		if (!done1 && check_digest1(digest)) {
			printf("Found 5 zeros! N = %d\n", num);
			hex_digest(digest);
			done1 = 1;
		}
		
		if (!done2 && check_digest2(digest)) {
			printf("Found 6 zeros! N = %d\n", num);
			hex_digest(digest);
			done2 = 1;
		}
	}
	
	return 0;
}