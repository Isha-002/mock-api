#include <stdio.h>

int main()
{
  char input[50];
  int count = 0;
  printf("counting and reversing string input:");
  fgets(input, 50, stdin);
  for (int i = 0; i < 50; i++)
  {

    if (input[i] != '\0')
    {
      count = count + 1;
      if (input[i] == '\n')
      {
        count = count - 1;
      }
    }
    else
    {
      break;
    }
    printf(" [%c]", input[i]);
  }
  printf("\nthe amount of characters: (%d)\n", count);
  char result[50] = {0};
  for (int i = 0; i < count; i++)
  {
    result[i] = input[count - i - 1];
    printf("%c", result);
  }
  printf("\n%s", result);

  return 0;
}
