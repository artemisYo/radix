#ifndef SLICE_IMPL
#define SLICE_IMPL
#include <stdbool.h>

#define Slice(type) struct __Slice_##type
#define slice_from(type, ptr, len) __Slice_##type##_from(ptr, len)
#define slice_is_empty(type, slice) __Slice_##type##_is_empty(slice)
#define slice_drop(type, slice, n) __Slice_##type##_drop(slice, n)
#define slice_next(type, slice) __Slice_##type##_next(slice)

#define DeclSlice(type) \
       Slice(type)      \
  { type * start        \
  ; type * end; };      \
Slice(type) __Slice_##type##_from(type * ptr, size_t len) {\
	return (Slice(type)) {.start = ptr, .end = ptr + len}; \
}                                                          \
void __Slice_##type##_drop(Slice(type) * slice, size_t n) {\
    slice->start += n;                                     \
    if (slice->start > slice->end) slice->start = slice->end;\
}                                                          \
bool __Slice_##type##_is_empty(Slice(type) slice) {        \
    return slice.start >= slice.end;                       \
}                                                          \
Option(type) __Slice_##type##_next(Slice(type) * slice) {  \
    if (slice_is_empty(type, *slice)) return None(type);   \
    type out = *slice->start;                              \
    slice->start++;                                        \
    return Some(type, out);                                \
}                                                          \

#endif
