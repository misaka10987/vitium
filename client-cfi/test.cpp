#include "vitium_client.h"
#include <string.h>
int main(){
    Conj x = get("https://httpbin.org/ip");
    printf(x.resp);
}