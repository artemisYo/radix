#ifndef SLICE_IMPL
#define SLICE_IMPL

#define Slice(type) struct __Slice_##type

#define DeclSlice(type) \
       Slice(type)      \
  { type * start        \
  ; type * end; };      \

// TODO: slice functions, i.e. `from`

#endif
