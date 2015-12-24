#include <stdio.h>
#include <string.h>

#define VOWELS 5
#define FORBIDDEN 4
#define MAXN 1000

char vowels[VOWELS] = {'a', 'e', 'i', 'o', 'u'};
char* forbidden[FORBIDDEN] = {"ab", "cd", "pq", "xy"};

char toLower(char c) {
	if (c >= 'A' && c <= 'Z')
		return c + 'a' - 'A';
		
	return c;
}

int isVowel(char c) {
	int i;
	c = toLower(c);
	
	for (i = 0; i < VOWELS; i++) {
		if (c == vowels[i])
			return 1;
	}
	
	return 0;
}

int isNice1(const char* s) {
	int i, j;
	int nr_vowels = 0, counter = 1;
	int contains_forbidden = 0;
	int len = strlen(s);
	
	for (i = 0; i < len; i++) {
		if (isVowel(s[i])) nr_vowels++;
		
		for (j = i + 1; counter < 2 && j < len && s[i] == s[j]; j++) {
			counter++;
		}
		
		if (counter < 2) counter = 1;
	}
	
	for (i = 0; i < FORBIDDEN && !contains_forbidden; i++) {
		if (strstr(s, forbidden[i]) != NULL) {
			contains_forbidden = 1;
		}
	}
	
	return counter >= 2 && nr_vowels >= 3 && !contains_forbidden;
}

int isNice2(const char* s) {
	int i;
	int len = strlen(s);
	int contains_pair = 0;
	int contains_repetition = 0;
	
	char subs[2 + 1] = {0};
	
	for (i = 0; i < len; i++) {
		if (!contains_pair && i < len - 3) {
			subs[0] = s[i];
			subs[1] = s[i + 1];
			
			if (strstr(s + i + 2, subs) != NULL) {
				contains_pair = 1;
			}
		}
		
		if (i < len - 2) {
			if (s[i] == s[i + 2]) {
				contains_repetition = 1;
			}
		}
	}
	
	return contains_pair && contains_repetition;
}

int main(void) {
	int nice = 0;
	int naughty = 0;
	char s[MAXN + 1];
	
	while (scanf("%s", s) != EOF) {
		if (isNice2(s)) {
			nice++;
		} else {
			naughty++;
		}
	}
	
	printf("Nice: %d, Naughty: %d\n", nice, naughty);

	return 0;
}