#include <stdio.h>

#define MAX(a, b) (((a) > (b)) ? (a) : (b))

#define SPOONS 100
#define MAX_ING 100
#define PROPERTIES 5

int calories; /* ignored if <= 0 */

int ingredients[MAX_ING][PROPERTIES];
int n; /* number of ingredients in the list */

int tmp_ings[PROPERTIES];
int max_score;

void calculate_score(int left, int index) {
	int i, j, score = 1;
	
	if (index == n) {
		for (i = 0; i < PROPERTIES - 1; i++) {
			score *= MAX(tmp_ings[i], 0);
		}
		
		if (score > max_score && (calories <= 0 || tmp_ings[PROPERTIES - 1] == calories))
			max_score = score;
			
		return;
	}
	
	for (i = 0; i <= left; i++) {
		if (calories > 0 && tmp_ings[PROPERTIES - 1] + i * ingredients[index][PROPERTIES - 1] > calories) 
			continue;
	
		for (j = 0; j < PROPERTIES; j++)
			tmp_ings[j] += i * ingredients[index][j];
			
		/* propagate # of spoons left to the next ingredient */
		calculate_score(left - i, index + 1);
		
		for (j = 0; j < PROPERTIES; j++)
			tmp_ings[j] -= i * ingredients[index][j];
	}
}

int main(void) {
	int i;
	
	while (scanf("%*s %*s %d, %*s %d, %*s %d, %*s %d, %*s %d", 
		&ingredients[n][0], 
		&ingredients[n][1], 
		&ingredients[n][2], 
		&ingredients[n][3], 
		&ingredients[n][4]) != EOF) {
		
		n++;
	}
	
	calculate_score(SPOONS, 0);
	printf("%d\n", max_score);
	
	calories = 500; /* set exact calorie count */
	max_score = 0; /* reset score */
	calculate_score(SPOONS, 0);
	printf("%d\n", max_score);

	return 0;
}