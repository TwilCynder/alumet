#include <stdint.h>

/**
 * @brief Cleans mojitos' state, freeing everything.
 * 
 */
void clean(void);

/**
 * @brief Initializes mojitos's state. The get_* are ready to be called immediately after this.   
 * 
 * @param argv arguments to be passed to mojitos. MUST BE ONLY MOJITOS ARGUMENTS, not including argv[0] (you can pass argv + 1 or &argv[1] from your main)
 * @return the number of metrics registered. This will be the size of arrays returned by get_* functions. 
 */
int init(char** argv);

/**
 * @brief Get the labels of the metrics measured as an array of strings
 * 
 * @return char** 
 */
char ** get_labels();

/**
 * @brief Makes all measurements and returns an array of values.
 * 
 * @return const uint64_t* 
 */
const uint64_t* get_values();