# MoojitOS Plugin
This plugin allows Alumet to make measurements using [MojitOS](https://gitlab.irit.fr/sepia-pub/mojitos), which comes with a wide variety of sensors for different system metrics (energy, memory, networking, etc), which will be listed below. 

Sensors must be explicitly enabled via a string of arguments in the config (which are the same arguments as the ones used by the CLI version of MojitOS).

### Config
```toml
[plugins.mojitos]
poll_interval = "1s" #Interval during measurements (all sensors are triggered simultaneously)
arguments = "-c -u"  #String of flags indicating which sensors to use
```

### Sensors

Here are the sensors that can be enabled, with the option you must add to the "arguments" property in the config to enable them. 

```
-r|--amd-rapl
	AMD RAPL (micro-joules)
-p|--perf-list <perf_list>
	performance counters
	perf_list is a coma separated list of performance counters.
	Ex: instructions,cache_misses
-i|--monitor-infiniband <infiniband_path>
	infiniband monitoring (if infiniband_path is X, tries to detect it automatically)
-u|--sysload
	system load
-d|--net-dev <net_dev>
	network monitoring (if network_device is X, tries to detect it automatically)
-n|--nvidia-gpu
	provides basic gpu information [clocks, memory, utilization, power, temperature].
-r|--intel-rapl
	INTEL RAPL (micro-joules)
-c|--cpu-temp
	processor temperature
-m|--memory
	Retrieves information about the memory via the syscall 'sysinfo(2)'.
-M|--memory-counters <memory_list>
	memory counters
	memory_list is a coma separated list of memory counters.
	Ex: Zswap,Zswapped
-L|--memory-list
	list the available memory counters and quit
-k|--likwid <perf_list>
	performance counters
	perf_list is a coma separated list of performance counters with associated register.
	Ex: FP_ARITH_INST_RETIRED_128B_PACKED_DOUBLE:PMC0,FP_ARITH_INST_RETIRED_SCALAR_DOUBLE:PMC1
-w|--list-likwid
	list the available performance counters and quit

```

Note that MojitOS is not always built with all its sensors, and that this plugin relies on a MojitOS build that may have been built without all the sensors. See the "Linking MojitOS" part below for more information. 

## Linking MojitOS
This plugin relies on `libmojitos`, which you can obtain in various way, the most simple being to build it yourself from MojitOS's code. Keep in mind that a mojitos build can include any subset of the sensors, which is controlled by the `configure.sh` script. Running this script with no arguments like below will enable all the sensors that work on your machine. Refer to MojitOS's documentation for more information about sensor selection. 

```bash
git clone https://gitlab.irit.fr/sepia-pub/mojitos
cd mojitos
chmod a+x ./configure.sh
./configure.sh
make libmojitos
make install
```

`libmojitos`, and its header, should be installed to `/usr/local/`, where this plugin's build system expects to find it.  
If you wish to use a build placed in a different location, you need to modify the build.rs script. The const variables at the top control the directories where `libmojitos` and the `mojitos.h` header are expected. 