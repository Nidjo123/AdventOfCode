#include <stdio.h>

int main(void) {
	int floor = 0;
	int counter = 1;
	int set = 0;
	char c;
	
	while ((c = getchar()) == '(' || c == ')') {
		if (c == '(')
			floor++;
		else if (c == ')')
			floor--;

		if (!set && floor == -1) {
			printf("Santa enters floor -1 at position %d!\n", counter);
			set = 1;
		}
		
		counter++;
	}
	
	printf("%d\n", floor);

	return 0;
}