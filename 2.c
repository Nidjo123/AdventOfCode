#include <stdio.h>

#define MIN(a, b) (((a)>(b)) ? (b) : (a))
#define N 3

int main(void) {
	int i, j;
	int dims[3];
	int side1, side2, side3;
	int volume;
	int tmp;
	long sumPaper = 0;
	long sumRibbon = 0;
	
	while (scanf("%dx%dx%d", dims, dims+1, dims+2) != EOF) {
		// one iteration of bubble sort works!
		for (i = 0; i < N - 1; i++) {
			if (dims[i] > dims[i + 1]) {
				tmp = dims[i + 1];
				dims[i + 1] = dims[i];
				dims[i] = tmp;
			}
		}
		
		volume = 1;
		
		for (i = 0; i < N; i++) {
			for (j = 0; j < N; j++) {
				if (i != j)
					sumPaper += dims[i] * dims[j];
			}
			volume *= dims[i];
		}
		
		sumPaper += dims[0] * dims[1];
		
		sumRibbon += volume + 2 * (dims[0] + dims[1]);
	}
	
	printf("Paper needed: %d\n", sumPaper);
	printf("Ribbon needed: %d\n", sumRibbon);

	return 0;
}