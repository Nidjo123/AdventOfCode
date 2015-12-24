#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define PRESENTS 29000000
#define PRIMES (PRESENTS / 10)

int *primes;
int np;

int house(int n) {
	int i, res;
	int pf, tpf, tmp = n;
	static int p_factors[10000], npf;
	
	if (n <= 1) { 
		res = 1;
	} else if (!primes[n]) {
		res = n + 1;
	} else {
		res = n + 1;
	
		npf = 0;
		
		while (n > 1) {
			
		}
	}
	
	return res * 10;
}

int main(void) {
	int i, j, presents, n = 1, is_prime;
	
	primes = (int*) malloc(PRIMES * sizeof(int));
	memset(primes, 0, PRIMES * sizeof(int));
	
	printf("DONE!\n");
	
	primes[0] = primes[1] = 0;
	
	for (i = 4; i < PRIMES; i += 2)
		primes[i] = 2;
	
	for (i = 3; i < PRIMES; i += 2) {
		if (primes[i]) continue;
		
		for (j = i << 1; j < PRIMES; j += i)
			if (!primes[j]) primes[j] = i;
	}
	
	printf("Done generating prime sieve!\n");
	
	for (i = 0; i < 20; i++) {
		printf("i = %d: %d\n", i + 1, house(i + 1));
	}
	free(primes);
	
	return 0;
}