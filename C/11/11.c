#include <stdio.h>

#define LENGTH 8
#define FORBIDDEN 3
#define INCREASING 3
#define TIMES_EXPIRED 2

int is_forbidden(char c) {
	static char forbidden[FORBIDDEN] = {'i', 'o', 'l'};
	int i;
	
	for (i = 0; i < FORBIDDEN; i++)
		if (c == forbidden[i])
			return 1;
			
	return 0;
}

int check_password(char* password) {
	int i, j;
	int increasing = 1;
	int forbidden = 0;
	int pairs = 0; 
	
	for (i = 0; i < LENGTH; i++) {
		if (is_forbidden(password[i])) {
			forbidden = 1;
			break;
		}
		
		if (increasing < 3) {
			/* haven't found increasing subsequence yet! */
			increasing = 1;
			j = i;
			while (j < LENGTH - 1 && password[j] == password[j + 1] - 1) {
				increasing++;
				j++;
			}
		}
		
		/* there's not point in checking just last two characters */
		if (!pairs && i < LENGTH - 3) {
			if (password[i] == password[i + 1]) {
				for (j = i + 2; j < LENGTH - 1; j++) {
					if (password[j] == password[j + 1] && password[i] != password[j])
						pairs = 1;
				}
			}
		}
	}
	
	return !forbidden && increasing >= 3 && pairs;
}

/* returns next password without forbidden characters */
void next(char* password) {
	int i, j;
	
	for (j = 0; j < LENGTH; j++) {
		if (is_forbidden(password[j])) {
			break;
		}
	}

	for (i = LENGTH - 1; i >= 0; i--) {
		if (i > j) {
			/* if it's after the forbidden char, just set it to 'a' */
			password[i] = 'a';
		} else if (password[i] == 'z') {
			password[i] = 'a';
		} else {
			password[i]++;
			break;
		}
	}
}

void next_valid(char* password) {
	next(password);
	
	while (!check_password(password))
		next(password);
}

int main(void) {
	int i;
	char password[LENGTH + 1];

	scanf("%s", password);
	
	for (i = 0; i < TIMES_EXPIRED; i++) {
		next_valid(password);
		printf("%s\n", password);
	}
	
	return 0;
}