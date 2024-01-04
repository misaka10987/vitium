#include<bits/stdc++.h>
class trait
{
private:
    int trait_num;
    int* trait_pointer[1024];
public:
    trait(int num_of_traits,std::ifstream load);
    ~trait();
};

trait::trait(int num_of_traits,std::ifstream load)
{
}

trait::~trait()
{
}
