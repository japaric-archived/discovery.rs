PROVIDE(nmi         = abort);
PROVIDE(hard_fault  = abort);
PROVIDE(mem_manage  = abort);
PROVIDE(bus_fault   = abort);
PROVIDE(usage_fault = abort);
PROVIDE(svc         = abort);
PROVIDE(debug_mon   = abort);
PROVIDE(pend_sv     = abort);
PROVIDE(sys_tick    = abort);

ENTRY(reset)
