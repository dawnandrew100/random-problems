#include <stdio.h>

void count_evens(int upper_limit);

int main() {
    count_evens(1000);
    return 0;
}

void count_evens(int upper_limit) {
  int i = 0;
  for (i = 0; i < upper_limit; i++) {
    if(i%2 == 0) {
        printf("%d\n", i);
    }
  }
}
