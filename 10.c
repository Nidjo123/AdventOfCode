#include <stdlib.h>
#include <stdio.h>
#include <string.h>

#define STARTLEN 1000
#define ITERATIONS 50
#define ITER1 40

typedef struct String_str {
	char* data;
	int size;
	int length;
} String;

String newString(int size) {
	String res;
	
	res.data = malloc(size);
	
	if (res.data == NULL)
		printf("Couldn't allocate memory for string!\n");
		
	res.data[0] = '\0';
	
	res.size = size;
	res.length = 0;
	
	return res;
}

void deleteString(String* string) {
	free(string->data);
}

void resizeString(String* string, int newSize) {
	string->data = realloc(string->data, newSize);
	string->size = newSize;
}

void copyString(String* dest, String* src) {
	int i;
	
	while (dest->size <= src->length) {
		resizeString(dest, dest->size << 1);
	}
	
	for (i = 0; i < src->length; i++) {
		dest->data[i] = src->data[i];
	}
	
	dest->data[i] = '\0';
	dest->length = src->length;
}

void appendString(String* dest, String* src) {
	int i;
	
	while (dest->size <= dest->length + src->length) {
		resizeString(dest, dest->size << 1);
	}
	
	for (i = 0; i < src->length; i++) {
		dest->data[dest->length + i] = src->data[i];
	}
	
	dest->data[dest->length + src->length] = '\0';
	dest->length += src->length;
}

void appendChars(String* dest, char s[]) {
	String tmp = *dest;
	appendString(dest, &tmp);
}

void appendChar(String* dest, char c) {
	if (dest->size <= dest->length + 1) {
		resizeString(dest, dest->size << 1);
	}
	
	dest->data[dest->length] = c;
	dest->data[dest->length + 1] = '\0';
	dest->length++;
}

void appendDigits(String* s, int count, char digit) {
	int factor = 1;
	
	while (count / factor >= 10) 
		factor *= 10;
		
	while (factor) {
		appendChar(s, '0' + count / factor);
		count %= factor;
		factor /= 10;
	}
	
	appendChar(s, digit);
}

void next(String* s) {
	int i, count, len =s->length;
	String tmp = newString(len + 1);
	char digit;
	
	count = 1;
	digit = s->data[0];
	
	for (i = 1; i < len; i++) {
		if (s->data[i] == digit) {
			count++;
		} else {
			appendDigits(&tmp, count, digit);
			count = 1;
			digit = s->data[i];
		}
	}
	
	appendDigits(&tmp, count, digit);
	
	copyString(s, &tmp);
	
	deleteString(&tmp);
}

int main(void) {
	int i;
	String string = newString(STARTLEN + 1);
	
	scanf("%s", string.data);
	
	string.length = strlen(string.data);

	for (i = 0; i < ITERATIONS; i++) {
		next(&string);
		if (i + 1 == ITER1)
			printf("Length after %d iterations: %d\n", ITER1, string.length);
	}
	
	printf("Length after %d iterations: %d\n", ITERATIONS, string.length);
	
	deleteString(&string);
	
	return 0;
}