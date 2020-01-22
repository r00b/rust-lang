#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <math.h>

// print the first n numbers that
// are palindromes
void palindrome(int n) {
	int begin = 11;  // smallest palindrome with 2 digits
	int counter = 0;
	while(counter < n){  // execute n (input) times
		int start = 0;
		int strSize = (int)((ceil(log10(begin))+1)*sizeof(char));
		char str[strSize];								// ensure that strSize is big enough
		sprintf(str,"%d",begin);  // convert int to str so we can index
		int end = strlen(str) - 1;
		while(start < end){
			if(str[start] == str[end]) {
				start++;  // move pointers closer to each other
				end--;
			} else {
				break;   // number can't be a palindrome
			}
		}
		if (start >= end ) {  // >= in case number has odd number of digits
			printf("%d\n",begin);
			counter++;
		}
		begin++; // check next number
	}
}

// main call
int main(int argc, char *argv[]){
	int n = atoi(argv[1]);
	palindrome(n);
}
