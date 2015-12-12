#include <stdio.h>

#define MAXN 1000
#define MAXDEPTH 1000

int is_int_char(int c) {
	return isdigit(c) || c == '-' || c == '+';
}

/* some super hackery, works (I hope!) only with valid JSON input */

char tmp[MAXN + 1];
int obj_sum[MAXDEPTH];
int last_c;

int sum_without_red(FILE* f, int obj_depth) {
	int i, value;
	int c = fgetc(f);
	
	if (c == EOF)
		return obj_sum[obj_depth];
	
	if (c == '{') {
		obj_sum[obj_depth + 1] = 0;
		return sum_without_red(f, obj_depth + 1);
	} else if (c == '"') {
		i = 0;
		while ((c = fgetc(f)) != '"')
			tmp[i++] = c;
		tmp[i] = '\0';
			
		if (!strcmp(tmp, "red") && obj_depth > 0 && last_c == ':') {
			/* skip whole object */
			int old_obj_depth = obj_depth;
			while (obj_depth >= old_obj_depth) {
				c = fgetc(f);
				
				if (c == '{') 
					obj_depth++;
				if (c == '}')
					obj_depth--;
			}

			obj_sum[old_obj_depth] = 0;
			return sum_without_red(f, old_obj_depth - 1);
		}
	} else if (c == '}') {
		/* add this depth sum to upper one */
		obj_sum[obj_depth - 1] += obj_sum[obj_depth];
		return sum_without_red(f, obj_depth - 1);
	} else if (is_int_char(c)) {
		i = 0;
		tmp[i++] = c;
		
		while (is_int_char(c = fgetc(f))) {
			tmp[i++] = c;
		}
		tmp[i] = '\0';
		
		sscanf(tmp, "%d", &value);
		
		obj_sum[obj_depth] += value;
		
		if (c == '}') {
			obj_sum[obj_depth - 1] += obj_sum[obj_depth];
			obj_depth--;
		}
			
		return sum_without_red(f, obj_depth);
	}
	
	last_c = c; /* so we can check the '["red"' case */
	
	/* else, just skip the character */
	return sum_without_red(f, obj_depth);
}

int main(void) {
	printf("%d\n", sum_without_red(stdin, 0));

	return 0;
}