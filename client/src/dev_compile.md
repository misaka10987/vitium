### The cpp ver is cpp20 
##### (though cpp17 is probably compatible)
If you wnat to use the script to compile, you would also need to install lib cpr to your system so that you may compile it.

Follow this link -> [cpr Github](https://github.com/libcpr/cpr)

### For Linux compilers
##### Using CMake (suggested)
We recommend Ninja for building. For more details, read CMake's docs.

##### Using the shell-script
Just run the "build.sh" and things should be fine. If you fail to find ncurses, you may try using pdcurses like win32 users. However, we do not offer support for this situation (as it is too rare). 

### For Windows compilers
#### note where to put PDCurses for win32 compile

##### If you wish to use CMake (suggested)
Please put the PDCurses folder parallel to the 'vitium' git repo (in the same parent folder).
You have to rename the extracted (or perhaps cloned) PDCurses repo (e.g. from "PDCurses-3.9") to "PDCurses".
Remember to compile the pdcurses library with make (follow their docs, dump it into ./wincon as defualt).

### The include tree
|Tree Top||||| 
| - | - | - | - | - |
|pdcurses/ncurses|||||
|curses_include.hpp||cpr.h|||
|ncurses_utils.cpp|map.cpp|-|||
|frontend.cpp||connect.cpp|-|registry.cpp|
|keyboard.cpp||-|||
|||main.cpp|||