#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::{bpf_get_current_pid_tgid, bpf_ktime_get_ns},
    macros::{map, tracepoint},
    maps::HashMap,
    programs::TracePointContext,
};
use ebpf_monitor_common::{PhiBitFlowEntry, SomaticMetrics};

#[map]
static METRICS: HashMap<u32, SomaticMetrics> = HashMap::with_max_entries(1, 0);

// phibitflow: chave = tgid (PID em userspace), valor = contadores cumulativos.
// 16384 entradas cobrem hosts densos; quando cheio, inserts novos são
// descartados (entrada existente continua acumulando).
#[map]
static PHIBITFLOW: HashMap<u32, PhiBitFlowEntry> = HashMap::with_max_entries(16384, 0);

// Offset do campo `count` (size_t) no formato do tracepoint sys_enter_read /
// sys_enter_write em x86_64: common(8B) + __syscall_nr(4B) + fd(4B) +
// buf(8B) = 24. Os dois tracepoints têm layout idêntico.
// Caveat: se o kernel expuser um formato diferente, o read_at falha ou
// retorna lixo — o bound-check abaixo descarta valores absurdos.
const TP_IO_COUNT_OFFSET: usize = 24;
const MAX_SINGLE_IO_BYTES: u64 = 1 << 32; // SSIZE_MAX real é bem menor; limite defensivo

#[inline(always)]
unsafe fn tp_count_arg(ctx: &TracePointContext) -> u64 {
    match ctx.read_at::<u64>(TP_IO_COUNT_OFFSET) {
        Ok(v) if v < MAX_SINGLE_IO_BYTES => v,
        _ => 0,
    }
}

#[inline(always)]
fn update_phibitflow(ctx: &TracePointContext, is_write: bool) {
    let tgid = (bpf_get_current_pid_tgid() >> 32) as u32;
    if tgid == 0 {
        return;
    }
    let bytes = unsafe { tp_count_arg(ctx) };
    let now = unsafe { bpf_ktime_get_ns() };
    unsafe {
        if let Some(entry) = PHIBITFLOW.get_ptr_mut(&tgid) {
            if is_write {
                (*entry).write_bytes += bytes;
                (*entry).write_calls += 1;
            } else {
                (*entry).read_bytes += bytes;
                (*entry).read_calls += 1;
            }
            (*entry).last_seen_ns = now;
        } else {
            let initial = PhiBitFlowEntry {
                write_bytes: if is_write { bytes } else { 0 },
                write_calls: if is_write { 1 } else { 0 },
                read_bytes: if is_write { 0 } else { bytes },
                read_calls: if is_write { 0 } else { 1 },
                last_seen_ns: now,
            };
            let _ = PHIBITFLOW.insert(&tgid, &initial, 0);
        }
    }
}

#[tracepoint(category = "syscalls", name = "sys_enter_execve")]
pub fn handle_execve(_ctx: TracePointContext) -> u32 {
    update_metrics(0); // 0 for execve
    0
}

#[tracepoint(category = "syscalls", name = "sys_enter_openat")]
pub fn handle_openat(_ctx: TracePointContext) -> u32 {
    update_metrics(1); // 1 for openat
    0
}

#[tracepoint(category = "syscalls", name = "sys_enter_connect")]
pub fn handle_connect(_ctx: TracePointContext) -> u32 {
    update_metrics(2); // 2 for connect
    0
}

#[tracepoint(category = "syscalls", name = "sys_enter_accept")]
pub fn handle_accept(_ctx: TracePointContext) -> u32 {
    update_metrics(2); // 2 for accept
    0
}

#[tracepoint(category = "syscalls", name = "sys_enter_read")]
pub fn handle_read(ctx: TracePointContext) -> u32 {
    update_metrics(3); // 3 for read
    update_phibitflow(&ctx, false);
    0
}

#[tracepoint(category = "syscalls", name = "sys_enter_write")]
pub fn handle_write(ctx: TracePointContext) -> u32 {
    update_metrics(3); // 3 for write
    update_phibitflow(&ctx, true);
    0
}

#[tracepoint(category = "syscalls", name = "sys_enter_clone")]
pub fn handle_clone(_ctx: TracePointContext) -> u32 {
    update_metrics(4); // 4 for clone (process pressure)
    0
}

#[inline(always)]
fn update_metrics(id: u32) {
    unsafe {
        if let Some(metrics) = METRICS.get_ptr_mut(&0) {
            match id {
                0 => (*metrics).execve_count += 1,
                1 => (*metrics).openat_count += 1,
                2 => (*metrics).tcp_connections += 1,
                3 => (*metrics).io_bursts += 1,
                4 => (*metrics).process_pressure += 1,
                _ => {}
            }
        } else {
            let mut initial = SomaticMetrics {
                execve_count: 0,
                openat_count: 0,
                tcp_connections: 0,
                io_bursts: 0,
                process_pressure: 0,
                last_timestamp: 0,
                // sovereign_word e pressure_level são zero-init aqui.
                // O userspace (OmniMind) os preenche via map update a cada ciclo.
                sovereign_word: [0u8; 32],
                pressure_level: 0,
            };
            match id {
                0 => initial.execve_count = 1,
                1 => initial.openat_count = 1,
                2 => initial.tcp_connections = 1,
                3 => initial.io_bursts = 1,
                4 => initial.process_pressure = 1,
                _ => {}
            }
            let _ = METRICS.insert(&0, &initial, 0);
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
