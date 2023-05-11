#include <dirent.h>
#include <stddef.h>

int main(int argc, char *argv[]) {
  int n = 0;
  struct dirent *d;
  DIR *dir = opendir(argv[1]);
  if (dir == NULL) //Not a directory or doesn't exist
    return 1;
  while ((d = readdir(dir)) != NULL) {
    if(++n > 2)
      break;
  }
  closedir(dir);
  if (n <= 2) //Directory Empty
    return 1;
  else
    return 0;
}
