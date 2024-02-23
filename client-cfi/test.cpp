#include "vitium_client.h"
#include <string.h>
int main(){
    Conj x = post("http://http.cat","mint");
    printf("%d\n",x.gotta);
    printf(x.resp);
}