#ifndef VEC_IMPL
#define VEC_IMPL

#include <stddef.h>
#include <string.h>

// vec typename
#define Vec(type) struct __Vec_##type

// vec functions
#define vec_init(type)      __Vec_##type##_init()
#define vec_boom(type, vec) __Vec_##type##_boom(vec)
#define vec_push(type, vec, elem) __Vec_##type##_push(vec, elem)
#define vec_pop(type, vec) __Vec_##type##_pop(vec)
#define vec_grow_for(type, vec, cap) __Vec_##type##_grow_for(vec, cap)
#define vec_index(type, vec, index) __Vec_##type##_index(vec, index)
#define vec_move_to_slice(type, vec) __Vec_##type##_move_to_slice(vec)

// generating a decl for the vec type
#define DeclVec(type)                                           \
DeclSlice(type);                                                \
DeclOption(type);                                               \
DeclPtr(type);                                                  \
IdDeclOption(ptr(type));                                        \
       Vec(type)                                                \
  { type * data                                                 \
  ; size_t len                                                  \
  ; size_t cap; };                                              \
Vec(type) __Vec_##type##_init();                                \
void __Vec_##type##_boom(Vec(type));                            \
size_t __Vec_##type##_push(Vec(type) *, type);                  \
Option(type) __Vec_##type##_pop(Vec(type) *);                   \
void __Vec_##type##_grow_for(Vec(type) *, size_t);              \
IdOption(ptr(type)) __Vec_##type##_index(Vec(type) *, size_t);  \
Vec(type) __Vec_##type##_init() {                               \
    type * data = alloc_many( type, 4 );                        \
    return ( Vec(type) ){ .data = data, .len = 0, .cap = 4};    \
}                                                               \
void __Vec_##type##_boom(Vec(type) vec) {                       \
    free(vec.data);                                             \
}                                                               \
size_t __Vec_##type##_push(Vec(type) * vec, type elem) {        \
    if (vec->len >= vec->cap) {                                 \
        vec_grow_for( type, vec, 1 );                           \
    }                                                           \
    *(vec->data + vec->len) = elem;                             \
    vec->len++;                                                 \
    return vec->len-1;                                          \
 }                                                              \
Option(type) __Vec_##type##_pop(Vec(type) * vec) {              \
    if (vec->len == 0) return None(type);                       \
    vec->len--;                                                 \
    return Some(type, *(vec->data + vec->len + 1));             \
}                                                               \
void __Vec_##type##_grow_for(Vec(type) * vec, size_t req) {     \
    vec->cap += vec->cap / 2 + req;                             \
    vec->data = realloc_many(type, vec->data, vec->cap);        \
}                                                               \
IdOption(ptr(type)) __Vec_##type##_index(Vec(type) * vec, size_t index) {\
    if (vec->len <= index) return IdNone(ptr(type));            \
    return IdSome(ptr(type), cast(type, vec->data + index));    \
}                                                               \
Slice(type) __Vec_##type##_move_to_slice(Vec(type) * vec) {     \
    type * slice = alloc_many(type, vec->len);                  \
    memcpy(slice, vec->data, vec->len * sizeof(type));          \
    Slice(type) out = slice_from(type, slice, vec->len);        \
    vec->len = 0;                                               \
    return out;                                                 \
}                                                               

#endif
