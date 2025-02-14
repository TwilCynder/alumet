#pragma once

#include <stdint.h>

typedef unsigned int (*initializer_t)(char *, void **);
typedef void (*labeler_t)(char **, void *);
typedef unsigned int (*getter_t)(uint64_t *, void *);
typedef void (*cleaner_t)(void *);

typedef struct Sensor {
    initializer_t init;
    getter_t get;
    cleaner_t clean;
    labeler_t label;
    int nb_opt;
} Sensor;