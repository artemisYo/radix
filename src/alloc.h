#ifndef ALLOC_DECL
#define ALLOC_DECL

#include <stdlib.h>
#define alloc(type) ( type * )malloc( sizeof( type ) )
#define alloc_many(type, count) ( type * ) calloc( count, sizeof( type ) )
#define realloc(type, ptr) ( type * ) realloc( ptr, sizeof( type ) )
#define realloc_many(type, ptr, count) ( type * ) reallocarray( ptr, count, sizeof( type ) )
// really only here in case the free function changes
#define free(ptr) free(ptr)

#endif
