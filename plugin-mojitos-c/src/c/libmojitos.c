#include <assert.h>
#include <inttypes.h>
#include <signal.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>
#include <unistd.h>
#include "util.h"

#define OPTPARSE_IMPLEMENTATION
//#define OPTPARSE_API static
#include "optparse.h"

/* optparse typedef */

int nb_defined_sensors = 0;

typedef struct Opt Opt;

#include "sensors.h"

Sensor sensors[NB_SENSOR];

#define NB_OPT 0
Optparse opts[NB_OPT + NB_SENSOR_OPT + 1];

unsigned int nb_sources = 0;
void **states = NULL;
getter_t *getter = NULL;
cleaner_t *cleaner = NULL;

unsigned int nb_sensors = 0;
char **labels = NULL;
uint64_t *values = NULL;


void clean(void)
{
    if (cleaner){
        for (unsigned int i = 0; i < nb_sources; i++) {
            cleaner[i](states[i]);
        }
    }

    if (nb_sources > 0) {
        free(getter);
        free(cleaner);
        //free(labels);
        free(values);
        free(states);
    }
}

void add_source(Sensor *cpt, char *arg)
{
    nb_sources++;
    initializer_t init = cpt->init;
    labeler_t labeler = cpt->label;
    getter_t get = cpt->get;
    cleaner_t clean = cpt->clean;

    states = realloc(states, nb_sources * sizeof(void *));
    int nb = init(arg, &states[nb_sources - 1]);

    if (nb == 0) {
        nb_sources--;
        states = realloc(states, nb_sources * sizeof(void *));
        return;
    }

    getter = realloc(getter, nb_sources * sizeof(void *));
    getter[nb_sources - 1] = get;
    cleaner = realloc(cleaner, nb_sources * sizeof(void *));
    cleaner[nb_sources - 1] = clean;

    labels = realloc(labels, (nb_sensors + nb) * sizeof(char *));
    labeler(labels + nb_sensors, states[nb_sources - 1]);

    values = realloc(values, (nb_sensors + nb) * sizeof(uint64_t));
    nb_sensors += nb;
}

int init(char** argv){

    init_sensors(opts, sensors, NB_OPT + NB_SENSOR_OPT, NB_OPT, &nb_defined_sensors);

    int opt;
    struct optparse options;
    options.permute = 0;

    optparse_init(&options, argv);
    while ((opt = optparse_long(&options, opts, NULL)) != -1){
        int ismatch = 0;
        int opt_idx = NB_OPT;
        for (int i = 0; i < nb_defined_sensors && !ismatch; i++) {
            for (int j = 0; j < sensors[i].nb_opt; j++) {
                if (opt == opts[opt_idx].shortname) {
                    ismatch = 1;
                    if (opts[opt_idx].fn != NULL) {
                        (void) opts[opt_idx].fn(NULL, 0);
                    } else {
                        add_source(&sensors[i], options.optarg);
                    }
                    break;
                }
                opt_idx++;
            }
        }
        if (!ismatch) {
            return -1;
        }
    }
    return nb_sensors;
}

char  * * get_labels(){
    return labels;
}

uint64_t const * get_values(){
    unsigned int current = 0;
    for (unsigned int i = 0; i < nb_sources; i++) {
        current += getter[i](&values[current], states[i]);
    }

    return values;
}