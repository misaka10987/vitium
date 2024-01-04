#include<bits/stdc++.h>

struct single_trait
{
    std::string trait_name;
    std::string trait_data;
};


class trait
{
private:
    int trait_num; //max at 1023
    int* trait_pointer[1024];
public:
    trait(int num_of_traits,std::ifstream load);
    ~trait();
};

trait::trait(int num_of_traits,std::ifstream load)
{
    //we will new every single trait as a single_trait struct and store the pointer toward it in the pointer array.
}

trait::~trait()
{
    //remember to delete...
}
