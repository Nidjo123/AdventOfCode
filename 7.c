#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <string.h>

#define MAXL 100
#define SIZE ('z' - 'a' + 1)

typedef enum {VALUE, AND, AND_VAL, OR, NOT, LSHIFT, RSHIFT, CONNECT} op_type;

typedef struct Gate_str {
	uint16_t value;
	op_type op;
	int a, b;
} Gate;

Gate wires[SIZE * SIZE];

op_type getOp(char* op) {
	if (!strcmp(op, "AND")) {
		return AND;
	} else if (!strcmp(op, "OR")) {
		return OR;
	} else if (!strcmp(op, "LSHIFT")) {
		return LSHIFT;
	} else if (!strcmp(op, "RSHIFT")) {
		return RSHIFT;
	}
}

int getPosition(char* wire) {
	int i, pos = 0;
	int len = strlen(wire);
	
	for (i = 0; i < len; i++) {
		if (i == len - 1) {
			pos += wire[i] - 'a';
		} else {
			pos += (wire[i] - 'a' + 1) * (len - i - 1) * SIZE;
		}
	}
	
	return pos;
}

int wire_done[SIZE << 1];
uint16_t getWireOutput(int wire) {
	Gate gate;
	uint16_t res;
	
	if (wire_done[wire]) return wires[wire].value;
	
	gate = wires[wire];
	
	switch (gate.op) {
		case VALUE:
			res = gate.value;
			break;
		case NOT:
			res = ~getWireOutput(gate.a);
			break;
		case AND:
			res = getWireOutput(gate.a) & getWireOutput(gate.b);
			break;
		case AND_VAL:
			res = gate.a & getWireOutput(gate.b);
			break;
		case OR:
			res = getWireOutput(gate.a) | getWireOutput(gate.b);
			break;
		case LSHIFT:
			res = getWireOutput(gate.a) << gate.b;
			break;
		case RSHIFT:
			res = getWireOutput(gate.a) >> gate.b;
			break;
		case CONNECT:
			res = getWireOutput(gate.a);
			break;
	}
	
	wire_done[wire] = 1;
	return (wires[wire].value = res);
}

int main(void) {
	char s1[MAXL + 1];
	char s2[MAXL + 1];
	char s3[MAXL + 1];
	char dest[MAXL + 1];
	Gate gate;
	
	while (scanf("%s %s", s1, s2) != EOF) {
		if (!strcmp(s2, "->")) {
			if (isdigit(s1[0])) {
				gate.op = VALUE;
				gate.value = atoi(s1);
			} else {
				gate.op = CONNECT;
				gate.a = getPosition(s1);
			}
		} else if (isupper(s1[0])) {
			// NOT
			gate.op = NOT;
			gate.a = getPosition(s2);
		} else {
			gate.op = getOp(s2);
			gate.a = getPosition(s1);
			
			if (gate.op == RSHIFT || gate.op == LSHIFT) {
				scanf("%d", &(gate.b));
			} else {
				scanf("%s", s3);
				gate.b = getPosition(s3);
				
				if (gate.op == AND && isdigit(s1[0])) {
					gate.op = AND_VAL;
					gate.a = atoi(s1);
				}
			}
		}
		
		if (gate.op != VALUE && gate.op != CONNECT) scanf("%*s");
		
		scanf("%s", dest);
		
		wires[getPosition(dest)] = gate;
	}
	
	printf("%u\n", getWireOutput(getPosition("a")));

	return 0;
}