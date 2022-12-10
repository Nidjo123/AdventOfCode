#include <stdio.h>

#define EGGNOG 150

int canisters[EGGNOG + 1];
int n;

int min_containers;
int ways;

int count(int left, int index, int used) {
	int i, j, res = 0;
		
	if (left == 0) {
		if (min_containers > used) {
			min_containers = used;
			ways = 1;
		} else if (min_containers == used) {
			ways++;
		}
			
		return 1;
	}
	
	if (left < 0 || index >= n)
		return 0;
	
	return count(left, index + 1, used) + count(left - canisters[index], index + 1, used + 1);
}

int main(void) {
	int x;
	
	while (scanf("%d", &x) != EOF) {
		canisters[n++] = x;
	}
	min_containers = n;
	
	printf("%d\n", count(EGGNOG, 0, 0));
	printf("Min containers: %d, ways to fill them: %d", min_containers, ways);
		
	return 0;
}