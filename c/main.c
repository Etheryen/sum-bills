#include <stdbool.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[]) {
  if (argc < 3) {
    fprintf(stderr, "Bad usage\n");
    return 1;
  }

  char *section = argv[1];
  size_t section_len = strlen(section);

  char *file_name = argv[2];

  char buf[BUFSIZ];
  FILE *f = fopen(file_name, "r");

  while (fgets(buf, sizeof(buf), f) && strncmp(buf, section, section_len))
    ;

  float sum = 0;
  while (fgets(buf, sizeof(buf), f) && *buf != '#')
    sum += atof(buf);

  fclose(f);

  printf("Sum is %.1fzł\n", sum);

  return 0;
}
