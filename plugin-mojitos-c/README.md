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
This branch does not provide a way to link MojitOS. You are most likely on the wrong branch. 