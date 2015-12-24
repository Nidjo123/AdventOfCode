#include <stdio.h>
#include <string.h>

#define MAXN 1000
#define MAXL 100

#define COORDS 4

int grid1[MAXN][MAXN];
int grid2[MAXN][MAXN];

int main(void) {
	int x, y;
	int i, j;
	int x1, y1, x2, y2;
	int toggle;
	int value1, value2;
	char s1[MAXL + 1];
	char s2[MAXL + 1];
	int turned_on1 = 0;
	int turned_on2 = 0;
	
	while (scanf("%s", s1) != EOF) {
		toggle = 0;
	
		if (!strcmp(s1, "toggle")) {
			toggle = 1;
			value2 = 2;
		} else {
			scanf("%s", s2);
			
			if (!strcmp(s2, "on")) {
				value1 = 1;
				value2 = 1;
			} else {
				value1 = 0;
				value2 = -1;
			}
		}
		
		scanf("%d,%d through %d,%d", &x1, &y1, &x2, &y2);
		
		for (x = x1; x <= x2; x++) {
			for (y = y1; y <= y2; y++) {
				if (toggle) {
					grid1[y][x] = !grid1[y][x];
					grid2[y][x] += value2;
				} else {
					grid1[y][x] = value1;
					grid2[y][x] += value2;
					
					if (grid2[y][x] < 0) {
						grid2[y][x] = 0;
					}
				}
			}
		}
	}
	
	for (x = 0; x  < MAXN; x++) {
		for (y = 0; y < MAXN; y++) {
			turned_on1 += grid1[y][x];
			turned_on2 += grid2[y][x];
		}
	}
	
	printf("Turned on 1: %d\n", turned_on1);
	printf("Turned on 2: %d\n", turned_on2);

	return 0;
}