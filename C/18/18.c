#include <stdio.h>

#define MAXN 100
#define DIRECTIONS 8
#define ON '#'
#define OFF '.'

#define STEPS 100

int n;
char grid[MAXN][MAXN + 1];

int dx[DIRECTIONS] = {+1, +1, +0, -1, -1, -1, +0, +1};
int dy[DIRECTIONS] = {+0, -1, -1, -1, +0, +1, +1, +1};

int neighbours_on(int x, int y) {
	int i, res = 0;
	int nx, ny;
	
	for (i = 0; i < DIRECTIONS; i++) {
		nx = x + dx[i];
		ny = y + dy[i];
		
		if (nx < 0 || ny < 0 || nx >= n || ny >= n) 
			continue;
			
		if (grid[ny][nx] == ON)
			res++;
	}
	
	return res;
}

int should_stay_on(int neighbours) {
	return neighbours == 2 || neighbours == 3;
}

int should_turn_on(int neighbours) {
	return neighbours == 3;
}

void copy_grid(char *gs, char *gd) {
	int x, y;
	
	for (y = 0; y < n; y++)
		for (x = 0; x < n; x++) 
			gd[y * (MAXN + 1) + x] = gs[y * (MAXN + 1) + x];
}

int count_nbs_on() {
	int x, y, res = 0;
	
	for (y = 0; y < n; y++)
		for (x = 0; x < n; x++)
			if (grid[y][x] == ON)
				res++;
				
	return res;
}

void print_grid(char *g) {
	int x, y;
	
	for (y = 0; y < n; y++) {
		for (x = 0; x < n; x++)
			printf("%c", g[y * (MAXN + 1) + x]);
		printf("\n");
	}
}

void update_santa(int x, int y, char *tmp_grid) {
	int nbs_on = neighbours_on(x, y);
	int index = y * (MAXN + 1) + x;
	
	if (grid[y][x] == ON && !should_stay_on(nbs_on)) {
		tmp_grid[index] = OFF;
	} else if (grid[y][x] == OFF && should_turn_on(nbs_on)) {
		tmp_grid[index] = ON;
	} else {
		tmp_grid[index] = grid[y][x];
	}
}

void update_conway(int x, int y, char *tmp_grid) {
	int nbs_on = neighbours_on(x, y);
	int index = y * (MAXN + 1) + x;
	
	if ((!x || x == n - 1) && (!y || y == n - 1)) {
		tmp_grid[index] = ON;
		return;
	}
	
	if (grid[y][x] == ON && (nbs_on < 2 || nbs_on > 3))
		tmp_grid[index] = OFF;
	else if (grid[y][x] == OFF && nbs_on == 3)
		tmp_grid[index] = ON;
	else
		tmp_grid[index] = grid[y][x];
}

void simulate(int steps, void (*func)(int, int, char*)) {
	int x, y, nbs_on;
	char tmp_grid[MAXN][MAXN + 1];
	
	while (steps--) {
		for (y = 0; y < n; y++) {
			for (x = 0; x < n; x++) {
				func(x, y, tmp_grid[0]);
			}
		}
		
		copy_grid(tmp_grid[0], grid[0]);
	}
}

int main(void) {
	int i, j;
	char tmp[MAXN][MAXN + 1];

	while (scanf("%s", grid[n]) != EOF) {
		n++;
	}
	
	copy_grid(grid[0], tmp[0]);
	
	simulate(STEPS, update_santa);
	printf("Santa: %d\n", count_nbs_on());
	
	copy_grid(tmp[0], grid[0]);
	
	/* all four corners should be on, always */
	grid[0][0] = ON;
	grid[0][n - 1] = ON;
	grid[n - 1][0] = ON;
	grid[n - 1][n - 1] = ON;
	
	simulate(STEPS, update_conway);
	printf("Conway: %d\n", count_nbs_on());
	
	return 0;
}