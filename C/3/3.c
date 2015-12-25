#include <stdio.h>
#include <stdlib.h>

typedef struct House_ {
	int x;
	int y;
} House;

typedef struct Node_ {
	House house;
	struct Node_* left;
	struct Node_* right;
} Node;

Node* newNode(House* house) {
	Node* node = malloc(sizeof(Node));
	
	node->house.x = house->x;
	node->house.y = house->y;
	node->left = NULL;
	node->right = NULL;
	
	return node;
}

void add(Node* root, House* house) {
	int sum, rootSum;
	Node* node;
	
	if (root == NULL) return;
	
	sum = house->x + house->y;
	rootSum = root->house.x + root->house.y;
	
	if (sum < rootSum) {
		if (root->left == NULL) {
			root->left = newNode(house);
		} else {
			add(root->left, house);
		}
	} else {
		if (house->x == root->house.x && house->y == root->house.y) {
			return;
		}
		
		if (root->right == NULL) {
			root->right = newNode(house);
		} else {
			add(root->right, house);
		}
	}
}

int size(Node* root) {
	if (root == NULL) return 0;
	
	return 1 + size(root->left) + size(root->right);
}

void printHouse(House* house) {
	printf("House (%d, %d)\n", house->x, house->y);
}

void inorder(Node* root) {
	if (root == NULL) return;
	
	inorder(root->left);
	printHouse(&(root->house));
	inorder(root->right);
}

// add all nodes from other tree
void addAll(Node* root, Node* other) {
	if (other == NULL) return;
	
	add(root, &(other->house));
	
	addAll(root, other->left);
	addAll(root, other->right);
}

void init(Node* root) {
	root->house.x = 0;
	root->house.y = 0;
	root->left = NULL;
	root->right = NULL;
}

void clear(Node* root) {
	if (root == NULL) return;
	
	clear(root->left);
	clear(root->right);
	
	free(root);
}

void updateHouse(char direction, House* house) {
	switch (direction) {
		case '^':
			house->y++;
			break;
		case '>':
			house->x++;
			break;
		case 'V':
		case 'v':
			house->y--;
			break;
		case '<':
			house->x--;
			break;
	}
}

int main(void) {
	House santaHouse = {0};
	House roboHouse = {0};
	char c;
	int turn = 0;
	Node* santa;
	Node* roboSanta;
	
	santa = malloc(sizeof(Node));
	roboSanta = malloc(sizeof(Node));
	
	init(santa);
	init(roboSanta);
	
	while (scanf("%c", &c) != EOF) {
		if (!turn) {
			updateHouse(c, &santaHouse);
			add(santa, &santaHouse);
		} else {
			updateHouse(c, &roboHouse);
			add(roboSanta, &roboHouse);
		}
		
		turn = !turn;
	}
	
	addAll(santa, roboSanta);
	
	printf("Houses with at least one present: %d\n", size(santa));

	clear(santa);
	clear(roboSanta);
	
	return 0;
}