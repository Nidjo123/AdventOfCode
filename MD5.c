#include "MD5.h"

#define N 64
#define CHUNK_SIZE 16

static unsigned s[N] = {7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
				5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
				4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
				6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21};
	
static unsigned K[N] = {0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 
				0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501,
				0x698098d8, 0x8b44f7af, 0xffff5bb1, 0x895cd7be,
				0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821,
				0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa,
				0xd62f105d, 0x02441453, 0xd8a1e681, 0xe7d3fbc8,
				0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed,
				0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a,
				0xfffa3942, 0x8771f681, 0x6d9d6122, 0xfde5380c,
				0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70,
				0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
				0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665,
				0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039,
				0x655b59c3, 0x8f0ccc92, 0xffeff47d, 0x85845dd1,
				0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1,
				0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391};

static int preprocessed_len;

static char* preprocess(char* msg) {
	int i;
	int len = strlen(msg);
	int bytes;
	char* result;
	int original_bits = len << 3, bits = original_bits + 1;
	
	if (bits % 512 < 448) {
		bits += 448 - bits % 512;
	} else if (bits % 512 > 448) {
		bits += bits % 512 + 448;
	}
	
	// length of preprocessed message
	bytes = (bits >> 3) + 8;
	
	result = malloc(sizeof(char) * bytes);
	
	if (result == NULL) return NULL;
	
	for (i = 0; i < len; i++) {
		result[i] = msg[i];
	}
	
	// padding
	
	result[i++] = 0x80;
	
	for (; i < bytes; i++) {
		result[i] = 0x00;
	}
	
	result[bytes - 8 + 3] = (original_bits >> 24) & 0xff;
	result[bytes - 8 + 2] = (original_bits >> 16) & 0xff;
	result[bytes - 8 + 1] = (original_bits >> 8) & 0xff;
	result[bytes - 8 + 0] = original_bits & 0xff;
	
	preprocessed_len = bytes;
	
	return result;
}

static unsigned leftRotate(unsigned x, unsigned c) {
	return (x << c) | (x >> (32 - c));
}

char* MD5(char* msg, char* digest_p) {
	int i, chunk, pos;
	unsigned a0 = 0x67452301;
	unsigned b0 = 0xefcdab89;
	unsigned c0 = 0x98badcfe;
	unsigned d0 = 0x10325476;
	unsigned A, B, C, D;
	unsigned F, g, dTemp, curr;
	unsigned M[CHUNK_SIZE] = {0};
	char* digest;
	char* _msg;
	
	_msg = preprocess(msg);
	
	if (_msg == NULL)  {
		free(digest);
		return NULL;
	}
	
	if (digest_p != NULL) {
		digest = malloc(DIGEST_LENGTH * sizeof(char));
		
		if (digest == NULL) return NULL;
	}
	
	// calculate digest
	for (chunk = 0; chunk < preprocessed_len * 8 / 512; chunk++) {
		pos = chunk << 6;
		
		for (i = 0; i < 16; i++) {
			M[i] = *((unsigned*)(_msg + pos));
			pos += 4;
		}
		
		A = a0;
		B = b0;
		C = c0;
		D = d0;
		
		for (i = 0; i < 64; i++) {
			if (i < 16) {
				F = (B & C) | ((~B) & D);
				g = i;
			} else if (i < 32) {
				F = (D & B) | ((~D) & C);
				g = (5 * i + 1) % 16;
			} else if (i < 48) {
				F = B ^ C ^ D;
				g = (3 * i + 5) % 16;
			} else {
				F = C ^ (B | (~D));
				g = (7 * i) % 16;
			}
			
			dTemp = D;
			D = C;
			C = B;
			B += leftRotate(A + F + K[i] + M[g], s[i]);
			A = dTemp;
		}
		
		a0 += A;
		b0 += B;
		c0 += C;
		d0 += D;
	}
	
	free(_msg);
	
	unsigned tmp[4] = {a0, b0, c0, d0};
	
	char* res = digest_p == NULL ? digest : digest_p;
	
	for (i = 0; i < 4; i++) {
		curr = tmp[i];
		
		pos = i << 2;
		res[pos + 3] = (curr >> 24) & 0xff;
		res[pos + 2] = (curr >> 16) & 0xff;
		res[pos + 1] = (curr >> 8) & 0xff;
		res[pos + 0] = curr & 0xff;
	}
	
	return digest_p == NULL ? digest : digest_p;
}

static void bin(char c) {
	int i, j = 7;
	
	for (i = 0; i < 8; i++, j--) {
		printf("%d", (c >> j) & 1);
	}
}

void print_bin(char* msg, int len) {
	int i;
	
	for (i = 0; i < len; i++) {
		bin(msg[i]);
	}
	
	printf("\n");
}

static int hex_d[] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'};

static char hex_digit(char c) {
	return hex_d[c & 0xf];
}

void hex_digest(char* digest) {
	int i;
	
	for (i = 0; i < DIGEST_LENGTH; i++) {
		printf("%c%c", hex_digit((digest[i] & 0xf0) >> 4), hex_digit(digest[i] & 0x0f));
	}
	
	printf("\n");
}
