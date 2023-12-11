#ifndef PTR_IMPL
#define PTR_IMPL
// a simple wrapper around a `type *` to allow writing `ptr(type)`
// instead, to avoid breaking macros
// as such it is also typedef'd

#define ptr(type) __Ptr_##type
#define ref(type, item) __Ptr_##type##_ref(&item)
#define cast(type, item) __Ptr_##type##_ref(item)
#define deref(type, ptr) __Ptr_##type##_deref(ptr)

#define DeclPtr(type)                      \
typedef struct ptr(type)                   \
  { type * item; } ptr(type);              \
ptr(type) __Ptr_##type##_ref(type * item) {\
    return ( ptr(type) ) { .item = item }; \
}                                          \
type __Ptr_##type##_deref(ptr(type) ptr) { \
    return *ptr.item;                      \
}

#endif // ptr_h_INCLUDED
