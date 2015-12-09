#include <stdio.h>
#include <string.h>

#define MAXNAME 20
#define MAXCONS 20
#define MAXLOCATIONS 20

typedef struct Location_str {
	char name[MAXNAME + 1];
	int nr_connections;
	struct Connection {
		struct Location_str* location;
		int distance;
	} connections[MAXCONS];
} Location;

Location locations[MAXLOCATIONS];
int size;

Location* path[MAXLOCATIONS];
Location* min_path[MAXLOCATIONS];
Location* max_path[MAXLOCATIONS];
int min_distance;
int max_distance;

void printPath(Location* path[], int len) {
	int i;

	for (i = 0; i < len; i++) {
		printf("%s", path[i]->name);
		if (i != len - 1)
			printf(" -> ");
	}
	printf("\n");
}

void tryPath(Location* location, int level, int path_distance) {
	int i, j, skip;
	
	path[level] = location;
	
	if (level == size - 1) {
		if (path_distance < min_distance) {
			min_distance = path_distance;
			for (i = 0; i < size; i++) 
				min_path[i] = path[i];
		} 
		
		if (path_distance > max_distance) {
			max_distance = path_distance;
			for (i = 0; i < size; i++)
				max_path[i] = path[i];
		}
		
		return;
	}
	
	for (i = 0; i < location->nr_connections; i++) {
		skip = 0;
		for (j = 0; j < level; j++) {
			if (!strcmp(path[j]->name, location->connections[i].location->name)) {
				skip = 1;
			}
		}
		
		if (skip) continue;
		
		tryPath(location->connections[i].location, level + 1, path_distance + location->connections[i].distance);
	}
}

void addConnection(Location* src, Location* dest, int distance) {
	src->connections[src->nr_connections].location = dest;
	src->connections[src->nr_connections].distance = distance;
	dest->connections[dest->nr_connections].location = src;
	dest->connections[dest->nr_connections].distance = distance;
	
	src->nr_connections++;
	dest->nr_connections++;
}

int newLocation(char* name) {
	strcpy(locations[size++].name, name);
	
	return size - 1;
}

int main(void) {
	int i, j;
	int found_src, found_dest;
	char src_name[MAXNAME + 1];
	char dest_name[MAXNAME + 1];
	int distance;
	
	while (scanf("%s to %s = %d", src_name, dest_name, &distance) != EOF) {
		found_src = found_dest = 0;
		min_distance += distance;
		
		for (i = 0; i < size; i++) {
			if (!strcmp(src_name, locations[i].name)) {
				found_src = 1;
				break;
			}
		}
		
		for (j = 0; j < size; j++) {
			if (!strcmp(dest_name, locations[j].name)) {
				found_dest = 1;
				break;
			}
		}
		
		if (!found_src) {
			i = newLocation(src_name);
		}
		
		if (!found_dest) {
			j = newLocation(dest_name);
		}
		
		addConnection(&locations[i], &locations[j], distance);
	}
	
	for(i = 0; i < size; i++)
		tryPath(locations + i, 0, 0);
	
	printf("Minimum path: %d\n", min_distance);
	printPath(min_path, size);
	
	printf("Maximum path: %d\n", max_distance);
	printPath(max_path, size);

	return 0;
}