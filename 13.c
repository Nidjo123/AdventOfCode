#include <stdio.h>
#include <string.h>

#define MAX(a, b) (((a) > (b)) ? (a) : (b))

#define MAXLEN 100
#define MAXN 20

int guests[MAXN][MAXN];
char names[MAXN][MAXLEN];
int guest_count;

int get_index_for(char* name) {
	int i;
	
	for (i = 0; i < guest_count; i++) {
		if (!strcmp(name, names[i]))
			return i;
	}
	
	guest_count;
	
	strcpy(names[guest_count], name);
	
	return guest_count++;
}

int combination[MAXN] = {-1};
int is_taken(int x, int level) {
	int i;
	
	for (i = 0; i < level; i++)
		if (combination[i] == x)
			return 1;
			
	return 0;
}

int max_happiness = 0xffffffff;
void calculate_happiness_difference(int level) {
	int i;
	
	if (level == guest_count) {
		/* calculate happiness */
		int diff = 0;
		
		for (i = 0; i < guest_count; i++) {
			diff += guests[combination[i]][combination[(i + 1) % guest_count]];
			diff += guests[combination[(i + 1) % guest_count]][combination[i]];
		}
		
		max_happiness = MAX(max_happiness, diff);
	}
	
	for (i = 0; i < guest_count; i++) {
		if (!is_taken(i, level)) {
			combination[level] = i;
			calculate_happiness_difference(level + 1);
		}
	}
}

int main(void) {
	int i;
	char name1[MAXLEN + 1];
	char name2[MAXLEN + 1];
	char action[MAXLEN + 1];
	int value;

	while (scanf("%s %*s %s %d %*s %*s %*s %*s %*s %*s %s", name1, action, &value, name2) != EOF) {
		name2[strlen(name2) - 1] = '\0';
		if (!strcmp(action, "lose"))
			value = -value;
			
		int index1 = get_index_for(name1);
		int index2 = get_index_for(name2);
			
		guests[index1][index2] = value;
	}
	
	calculate_happiness_difference(0);
	
	printf("%d\n", max_happiness);

	return 0;
}