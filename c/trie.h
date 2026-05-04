#ifndef TRIE_H
#define TRIE_H

/* NOTE(segfault): Description
  This is a stb single header trie library for C. Simple by design.
  The error logging is internal so no need for users to manually call it.
  The error log call back can be customized via a custom call back to enable it
  just #define TR_CUSTOM_CALLBACK.
*/

/* Example
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
*/

// NOTE(segfault): Default alphabet count
#define TR_ALPHABET_SIZE 26

// NOTE(segfault): Forward declartion of structures;
typedef struct TR_trie_t TR_trie_t;
typedef struct TR_trie_node_t TR_trie_node_t;

// NOTE(segfault): Enum for error
typedef enum
{
    TR_CREATE_NODE_FAILURE = 1,
    TR_GET_NULL_NODE_ERROR,
    TR_GET_NOT_NODE_ERROR,
    TR_SUCESS = 0,
} TR_error_t;

// NOTE(segfault): This is a function pointer callback used by our trie structure
// To get the errors if we incounter any because I thought what would make a better api rather than keeping track of a global error var. Note that this function does exit if an error is found so be wary of calling this function.

// NOTE(segfault): By default we have a default callback for error handeling if you
// wish to customize it you can enum you own custom call back with a macro
// #define TR_CUSTOM_CALLBACK.
typedef void (*TR_error_callback)(TR_error_t err, const char *context);


struct TR_trie_node_t
{
    TR_trie_node_t *children[TR_ALPHABET_SIZE];
    /* NOTE(segfault): I was a bit lazy and doing bool is too much so
    / for now I will use int -1 to represent not end and 0 to represent end.
    */
    int is_end;
    TR_error_t err;
};

struct TR_trie_t
{
    TR_trie_node_t *root;
    TR_error_callback on_error;
};


// NOTE(segfault): Creates a trie.
// Returns a trie node if success returns NULL if failure
TR_trie_t *TR_create();

// NOTE(segfault): Destroy's an existing trie.
void TR_destroy(TR_trie_t *trie);

// NOTE(segfault): Inserts a key into an existing trie.
// return TR_SUCESS if inserted. Returns TR_CREATE_NODE_FAILURE if it could not
// create a node to be inserted
TR_error_t TR_insert(TR_trie_t *trie, const char *key);

// NOTE(segfault):  Gets an element from the trie.
// Return an existing node if found else returns NULL.
TR_trie_node_t *TR_get(TR_trie_t *trie, const char *key);

// NOTE(segfault): Checks the existense of a prefix of chars in the trie
// Returns TS_SUCCESS if found else returns TR_GET_NULL_NODE_ERROR.
TR_error_t TR_prefix_search(TR_trie_t *trie, const char *key);

// NOTE(segfault): Delete a node from the trie using a key
TR_error_t TR_delete(TR_trie_t *trie, const char *key);

#if defined(TRIE_IMPLEMENTATION)
#include <stdio.h>
#include <stdlib.h>
#include <assert.h>
#include <string.h>

static void TR_default_error_handler(TR_error_t err, const char *ctx)
{
    fprintf(stderr, "[TRIE ERROR %d] %s\n", err, ctx ? ctx : "");
}

static TR_trie_node_t *TR_create_node(void)
{
    TR_trie_node_t *node = calloc(1, sizeof(TR_trie_node_t));
    if (!node) return NULL;

    node->is_end = 0;
    node->err = TR_SUCESS;
    return node;
}

static void TR_report(TR_trie_t *trie, TR_error_t err, const char *ctx)
{
    if (trie && trie->on_error)
        trie->on_error(err, ctx);
}

static TR_trie_t *TR_create_cb(TR_error_callback cb)
{
    TR_trie_t *trie = calloc(1, sizeof(TR_trie_t));
    if (!trie) return NULL;

    trie->root = TR_create_node();
    if (!trie->root)
    {
        free(trie);
        return NULL;
    }

#ifdef TR_CUSTOM_CALLBACK
    trie->on_error = cb;
#else
    (void)cb;
    trie->on_error = TR_default_error_handler;
#endif

    return trie;
}

TR_trie_t *TR_create(void)
{
    return TR_create_cb(0);
}

void TR_destroy_node(TR_trie_node_t *node)
{
    if (!node) return;

    for (int i = 0; i < TR_ALPHABET_SIZE; i++)
        TR_destroy_node(node->children[i]);

    free(node);
}

void TR_destroy(TR_trie_t *trie)
{
    if (!trie) return;

    TR_destroy_node(trie->root);
    free(trie);
}

TR_error_t TR_insert(TR_trie_t *trie, const char *key)
{
    if (!trie || !key)
    {
        TR_report(trie, TR_CREATE_NODE_FAILURE, "insert: null input");
        return TR_CREATE_NODE_FAILURE;
    }

    TR_trie_node_t *cur = trie->root;

    for (size_t i = 0; key[i]; i++)
    {
        if (key[i] < 'a' || key[i] > 'z')
        {
            TR_report(trie, TR_CREATE_NODE_FAILURE, "insert: invalid char");
            return TR_CREATE_NODE_FAILURE;
        }

        int idx = key[i] - 'a';

        if (!cur->children[idx])
        {
            cur->children[idx] = TR_create_node();
            if (!cur->children[idx])
            {
                TR_report(trie, TR_CREATE_NODE_FAILURE, "malloc failed");
                return TR_CREATE_NODE_FAILURE;
            }
        }

        cur = cur->children[idx];
    }

    cur->is_end = 1;
    return TR_SUCESS;
}

TR_trie_node_t *TR_get(TR_trie_t *trie, const char *key)
{
    if (!trie || !key)
    {
        TR_report(trie, TR_GET_NULL_NODE_ERROR, "get: null input");
        return NULL;
    }

    TR_trie_node_t *cur = trie->root;

    for (size_t i = 0; key[i]; i++)
    {
        int idx = key[i] - 'a';

        if (idx < 0 || idx >= TR_ALPHABET_SIZE || !cur->children[idx])
        {
            TR_report(trie, TR_GET_NULL_NODE_ERROR, "get: not found");
            return NULL;
        }

        cur = cur->children[idx];
    }

    if (!cur->is_end)
    {
        TR_report(trie, TR_GET_NOT_NODE_ERROR, "get: prefix only");
        return NULL;
    }

    return cur;
}

TR_error_t TR_prefix_search(TR_trie_t *trie, const char *key)
{
    if (!trie || !key)
    {
        TR_report(trie, TR_GET_NULL_NODE_ERROR, "prefix: null input");
        return TR_GET_NULL_NODE_ERROR;
    }

    TR_trie_node_t *cur = trie->root;

    for (size_t i = 0; key[i]; i++)
    {
        int idx = key[i] - 'a';

        if (idx < 0 || idx >= TR_ALPHABET_SIZE || !cur->children[idx])
            return TR_GET_NULL_NODE_ERROR;

        cur = cur->children[idx];
    }

    return TR_SUCESS;
}

static int TR_delete_rec(TR_trie_node_t *node, const char *key, size_t depth)
{
    if (!node) return 0;

    if (key[depth] == '\0')
    {
        node->is_end = 0;
    }
    else
    {
        int idx = key[depth] - 'a';

        if (TR_delete_rec(node->children[idx], key, depth + 1))
        {
            free(node->children[idx]);
            node->children[idx] = NULL;
        }
    }

    if (node->is_end) return 0;

    for (int i = 0; i < TR_ALPHABET_SIZE; i++)
        if (node->children[i]) return 0;

    return 1;
}

TR_error_t TR_delete(TR_trie_t *trie, const char *key)
{
    if (!trie || !key)
    {
        TR_report(trie, TR_GET_NULL_NODE_ERROR, "delete: null input");
        return TR_GET_NULL_NODE_ERROR;
    }

    TR_delete_rec(trie->root, key, 0);
    return TR_SUCESS;
}

#endif

#endif //TRIE_H
