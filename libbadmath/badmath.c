#include <stdio.h>
#include "badmath.h"

float bad_add(float v1, float v2){
    printf("I know how to add %f and %f", v1, v2);
    return v1 + v2 + 10.0;
}