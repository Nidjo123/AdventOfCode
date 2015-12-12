#include <stdio.h>
#include <string.h>

#define MAXN 1000

int is_int_char(char c) {
	return isdigit(c) || c == '-' || c == '+';
}

int main(void) {
	int i, j, len, start = 0;
	int value, sum = 0, continue_last = 0;
	char s[MAXN + 1];
	char last_s[MAXN + 1];
	
	while (fgets(s, MAXN + 1, stdin) != NULL) {
		len = strlen(s);
		
		if (continue_last) {
			continue_last = 0;
			
			if (is_int_char(s[0])) {
				/* continue last */
				
				for (i = 0; i < j - start; i++) {
					last_s[i] = last_s[start + i];
					printf("%c", last_s[start+i]);
				}
					
				j = 0;
				while (is_int_char(s[j])) 
					last_s[i++] = s[j++];
					
				sscanf(last_s, "%d", &value);
				
				start = j;
			} else {
				sscanf(last_s + start, "%d", &value);
				start = 0;
			}
			
			sum += value;
		} else {
			start = j = 0;
		}
		
		for (i = start; i < len; i++) {
			if (is_int_char(s[i])) {
				j = i + 1;
				
				while (j < len && is_int_char(s[j]))
					j++;
					
				if (j == len) {
					continue_last = 1;
					start = i;
					strncpy(last_s, s, MAXN + 1);
					break;
				}
				
				sscanf(s + i, "%d", &value);
				
				sum += value;
				
				i = j; /* skip number */
			}
		}
	}
	
	printf("%d\n", sum);

	return 0;
}