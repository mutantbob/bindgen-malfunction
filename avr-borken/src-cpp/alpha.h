#include "base.h"

class Alpha: Base
{
#ifdef MALFUNCTION
    int bar;
#else
    short bar;
    short bacon;
#endif
 public:
    int f() { return bar; }
};
