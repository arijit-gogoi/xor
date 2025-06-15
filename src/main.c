#define KEY 'X'
#define CLEARTEXT 'A'

#include <stdio.h>

void printchar(char c, char*text) {
  printf("%s = '%c' (0x%x)\n", text, c, c);
  return;
}

int main(void) {
  char cleartext, key, ciphertext, deciphertext;
  cleartext = CLEARTEXT;
  printchar(cleartext, "cleartext");
  key = KEY;
  printchar(key, "key");

  ciphertext = cleartext ^ key;
  printchar(ciphertext, "chphertext");

  deciphertext = ciphertext ^ key;
  printchar(deciphertext, "deciphertext");

  return 0;
}
