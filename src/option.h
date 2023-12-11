#ifndef OPTION_IMPL
#define OPTION_IMPL
#include <stdbool.h>

// Id* used to ensure the type is expanded before being inlined
#define IdOption(type) Option(type)
#define Option(type) struct __Option_##type

#define IdNone(type) None(type)
#define None(type) __Option_##type##_none()
#define IdSome(type, item) Some(type, item)
#define Some(type, item) __Option_##type##_some(item)

#define IdDeclOption(type) DeclOption(type)
#define DeclOption(type) \
       Option(type)      \
  { bool is_ok           \
  ; type data; };        \
Option(type) __Option_##type##_none() {\
    return ( Option(type) ){ .is_ok = false, .data = {0} };\
}\
Option(type) __Option_##type##_some(type item) {\
    return ( Option(type) ){ .is_ok = true, .data = item };\
}\


#endif
