#include <stdio.h>

#define MIN(a, b) (((a) < (b)) ? (a) : (b))
#define MAX(a, b) (((a) > (b)) ? (a) : (b))

#define TIME 2503
#define MAXLEN 50
#define NR_REINDEERS 9

typedef struct {
	char name[MAXLEN + 1];
	int speed;
	int fly_time;
	int rest_time;
	int distance;
	int points;
} Reindeer;

int calculate_distance(Reindeer* reindeer, int time) {
	int sum_time = reindeer->fly_time + reindeer->rest_time;
	int whole = time / sum_time;
	
	int res = (whole * reindeer->fly_time + MIN(reindeer->fly_time, time - sum_time * whole)) * reindeer->speed;
	
	return res;
}

int main(void) {
	int i, t, lead_distance = -1;
	Reindeer reindeers[NR_REINDEERS] = {0};
	Reindeer* max_distance = &reindeers[0];
	Reindeer* max_points = &reindeers[0];
	
	for (i = 0; i < NR_REINDEERS; i++)
		scanf("%s can fly %d km/s for %d seconds, but then must rest for %d seconds.", reindeers[i].name, &reindeers[i].speed, &reindeers[i].fly_time, &reindeers[i].rest_time);
	
	for (t = 1; t <= TIME; t++) {
		for (i = 0; i < NR_REINDEERS; i++) {
			reindeers[i].distance = calculate_distance(&reindeers[i], t);
			if (reindeers[i].distance > lead_distance)
				lead_distance = reindeers[i].distance;
		}
		
		for (i = 0; i < NR_REINDEERS; i++)
			if (reindeers[i].distance == lead_distance) 
				reindeers[i].points++;
		
		if (max_distance->points > max_points->points)
			max_points = max_distance;
	}
	
	for (i = 1; i < NR_REINDEERS; i++) {
		if (reindeers[i].distance > max_distance->distance)
			max_distance = &reindeers[i];
			
		if (reindeers[i].points > max_points->points)
			max_points = &reindeers[i];
	}
	
	printf("Reindeer with maximum distance: %s (%d km)\n", max_distance->name, max_distance->distance);
	printf("Reindeer with the most points: %s (%d points)\n", max_points->name, max_points->points);

	return 0;
}