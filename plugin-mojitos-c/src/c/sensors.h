#include "load.h"
#include "memory_counters.h"
#include "temperature.h"
#include "counters.h"
#include "rapl.h"
#include "network.h"
#include "memory.h"

#define NB_SENSOR 7
#define NB_SENSOR_OPT 9

void init_sensors(Optparse *opts, Sensor *sensors, size_t len, size_t offset, int *nb_defined)
{
    int opt_idx = offset;
    for (int i = 0; i < load.nb_opt; i++) {
        opts[opt_idx++] = load_opt[i];
    }
    sensors[(*nb_defined)++] = load;
    for (int i = 0; i < memory_counters.nb_opt; i++) {
        opts[opt_idx++] = memory_counters_opt[i];
    }
    sensors[(*nb_defined)++] = memory_counters;
    for (int i = 0; i < temperature.nb_opt; i++) {
        opts[opt_idx++] = temperature_opt[i];
    }
    sensors[(*nb_defined)++] = temperature;
    for (int i = 0; i < counters.nb_opt; i++) {
        opts[opt_idx++] = counters_opt[i];
    }
    sensors[(*nb_defined)++] = counters;
    for (int i = 0; i < rapl.nb_opt; i++) {
        opts[opt_idx++] = rapl_opt[i];
    }
    sensors[(*nb_defined)++] = rapl;
    for (int i = 0; i < network.nb_opt; i++) {
        opts[opt_idx++] = network_opt[i];
    }
    sensors[(*nb_defined)++] = network;
    for (int i = 0; i < memory.nb_opt; i++) {
        opts[opt_idx++] = memory_opt[i];
    }
    sensors[(*nb_defined)++] = memory;
    assert((offset + *nb_defined) <= len);
}
