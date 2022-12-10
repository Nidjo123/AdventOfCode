#include <stdio.h>
#include <string.h>

#define HEXDIGITS 16
#define MAXL 1000

char hex_digits[HEXDIGITS] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'};

int isHexDigit(char c) {
	int i;
	c = tolower(c);
	
	for (i = 0; i < HEXDIGITS; i++) {
		if (c == hex_digits[i]) {
			return 1;
		}
	}
	
	return 0;
}

int isHexASCII(char* s) {
	if (strlen(s) >= 4) {
		return s[0] == '\\' && (s[1] == 'x' || s[1] == 'X') && isHexDigit(s[2]) && isHexDigit(s[3]);
	}
	
	return 0;
}

int isQuote(char* s) {
	if (strlen(s) >= 2) {
		return s[0] == '\\' && s[1] == '"';
	}
	
	return 0;
}

int isBackslash(char* s) {
	if (strlen(s) >= 2) {
		return s[0] == '\\' && s[1] == '\\';
	}
	
	return 0;
}

int getUnencodedLength(char* s) {
	int i, len = strlen(s);
	int res = 0;
	
	for (i = 0; i < len; i++) {
		if (s[i] == '\\') {
			if (isBackslash(s + i) || isQuote(s + i)) {
				i += 2;
			} else if (isHexASCII(s + i)) {
				i += 4;
			}
			
			i--; // i++!
		}
		
		res++;
	}
	
	return res - 2; // 2 * "
}

int getEncodedLength(char* s) {
	int i, len = strlen(s);
	int res = len;
	
	for (i = 0; i < len; i++) {
		if (s[i] == '\\' || s[i] == '"') {
			res++;
		}
	}
	
	return res + 2; // 2 * "
}

int main(void) {
	char s[MAXL + 1];
	int original_length = 0;
	int unencoded_length = 0;
	int encoded_length = 0;

	while (scanf("%s", s) != EOF) {
		original_length += strlen(s);
		unencoded_length += getUnencodedLength(s);
		encoded_length += getEncodedLength(s);
	}
	
	printf("Saved: %d\nSpent: %d\n", original_length - unencoded_length, encoded_length - original_length);
	
	return 0;
}