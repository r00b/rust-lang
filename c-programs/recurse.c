#include <stdio.h>
#include <stdlib.h>

// apply f(N)=3*(N+1)+f(N-1)-1 to N
int function(int N) {
  if (N == 0) return 1;  // base case
  int result = 3 * (N+1) + function(N-1) - 1;  // recursive call
  return result;
}

// main call
int main(int argc, char *argv[]) {
  int N = atoi(argv[1]);  // convert input to int
  int f = function(N);
  printf("%d\n",f);  // print result
}
