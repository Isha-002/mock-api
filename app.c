#include <stdio.h>

int main() {
  int age;
  printf("enter your age:");
  scanf("%d", &age);
  if (age > 10) {
    printf("hello\nworld. im %d", age);
  } else {
    printf("hello kiddo ur %d", age);
  }
  

  return 0;
}