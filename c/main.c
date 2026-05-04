#define TRIE_IMPLEMENTATION
#include "trie.h"
#include <stdio.h>

int main(void)
{
    TR_trie_t *trie = TR_create();
    if (!trie) return 1;

    TR_insert(trie, "hello");
    TR_insert(trie, "helium");
    TR_insert(trie, "heat");

    if (TR_get(trie, "hello")) printf("hello found\n");
    if (!TR_get(trie, "heaven")) printf("heaven not found\n");

    if (TR_prefix_search(trie, "he") == TR_SUCESS)
        printf("prefix he exists\n");

    TR_delete(trie, "helium");
    if (!TR_get(trie, "helium")) printf("helium deleted\n");

    TR_destroy(trie);

    return 0;
}
