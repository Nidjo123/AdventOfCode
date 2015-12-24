#include <stdio.h>
#include <string.h>

#define MAXL 100
#define MAXN 1000
#define PER_AUNT 3

#define CHECK_RANGES 1

#define RANGE_COUNT 2

typedef enum {LT, GT, EQ} Compare;

typedef struct {
	char name[MAXL + 1];
	int count;
} Belonging;

Belonging belongings[MAXN];
int n; /* number of belongings */

char *greater_than[RANGE_COUNT] = {"cats", "trees"};
char *less_than[RANGE_COUNT] = {"pomeranians", "goldfish"};

void remove_last(char *s) {
	int len = strlen(s);
	
	if (len > 0)
		s[len - 1] = '\0';
}

void remove_last_all(char *s, int rows, int cols) {
	int i;
	
	for (i = 0; i < rows; i++)
		remove_last(s + i * cols);
}

int strcmp_any(const char *s, char *ss[], int n) {
	int i;
	
	for (i = 0; i < n; i++) 
		if (!strcmp(s, ss[i]))
			return 1;
			
	return 0;
}

int check_aunt_sim(char *s, int *count, int ranges) {
	int i, j, index, x = 0;
	Compare type;
	
	for (i = 0; i < PER_AUNT; i++) {
		index = i * (MAXL + 1);
	
		for (j = 0; j < n; j++) {
			if (!strcmp(s + index, belongings[j].name)) {
				break;
			}
		}
		
		type = EQ;
		
		if (ranges == CHECK_RANGES) {		
			if (strcmp_any(s + index, greater_than, RANGE_COUNT))
				type = GT;
			else if (strcmp_any(s + index, less_than, RANGE_COUNT)) 
				type = LT;
		}
		
		if (type == EQ && count[i] == belongings[j].count) {
			x++;
		} else if (type == GT && count[i] > belongings[j].count) {
			x++;
		} else if (type == LT && count[i] < belongings[j].count) {
			x++;
		}
	}
	
	return x;
}

int main(void) {
	int id, sim, sim_r, max_sim, max_sim_r;
	int real_id, real_id_r;
	char tmp[PER_AUNT][MAXL + 1];
	int count[PER_AUNT];

	/* first input belongings and their numbers */
	while (scanf("%s", belongings[n].name) == 1) {
		if (!strcmp("Sue", belongings[n].name))
			break;
		scanf("%d", &belongings[n].count);
		remove_last(belongings[n].name);
		n++;
	}
	
	n--;
	
	scanf("%d%*c %s %d, %s %d, %s %d, %s %d", 
		&id, tmp, count, tmp+1, count+1, tmp+2 , count+2, tmp+3, count+3);
		
	remove_last_all(tmp[0], PER_AUNT, MAXL + 1);
	max_sim = check_aunt_sim(tmp[0], count, !CHECK_RANGES);
	max_sim_r = check_aunt_sim(tmp[0], count, CHECK_RANGES);
	real_id = real_id_r = 1;
	
	while (scanf("%*s %d%*c %s %d, %s %d, %s %d, %s %d", 
		&id, tmp, count, tmp+1, count+1, tmp+2 , count+2, tmp+3, count+3) != EOF) {
		
		remove_last_all(tmp[0], PER_AUNT, MAXL + 1);
		
		sim = check_aunt_sim(tmp[0], count, !CHECK_RANGES);
		sim_r = check_aunt_sim(tmp[0], count, CHECK_RANGES);
		
		if (sim > max_sim) {
			max_sim = sim;
			real_id = id;
		}
		
		if (sim_r > max_sim_r) {
			max_sim_r = sim_r;
			real_id_r = id;
		}
	}
	
	printf("Real Aunt Sue without ranges: %d\n", real_id);
	printf("Real Aunt Sue with ranges: %d\n", real_id_r);
	
	return 0;
}