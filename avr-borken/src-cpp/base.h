#ifndef Base_h_
#define Base_h_


class Base
{
private:
    int maple;
 public:
    Base() : maple(7) {}
    virtual int f() =0;
};

#endif Base_h_
