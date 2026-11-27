#![no_std]

// Shared types between eBPF and userspace go here.
//
// sovereign_word: lexema transatlântico injetado pelo userspace (OmniMind)
// que descreve semanticamente o estado do sistema no momento da medição.
// Exemplos: "KU-LUMU-execve-tatá" (execve em pressão), "MA-LOZI-mem-katu" (memória saudável)
// O kernel carrega o número; o OmniMind devolve o nome.
#[cfg_attr(feature = "user", derive(serde::Serialize, Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SomaticMetrics {
    pub execve_count: u64,
    pub openat_count: u64,
    pub tcp_connections: u64,
    pub io_bursts: u64,
    pub process_pressure: u64,
    pub last_timestamp: u64,
    /// Lexema soberano atual — 32 bytes UTF-8, preenchido pelo userspace OmniMind.
    /// eBPF não gera o lexema (sem alloc), mas o lê e reenvia no mesmo map.
    pub sovereign_word: [u8; 32],
    /// 0=normal 1=pressure 2=critical — derivado do lexema (tatá=1, atã=2)
    pub pressure_level: u8,
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for SomaticMetrics {}

// phibitflow: contabilidade física de bits por processo (tgid).
// Preenchido pelos tracepoints sys_enter_read/sys_enter_write usando o
// argumento `count` da syscall — bytes reais entregues ao VFS pelo processo.
// write_bytes/read_bytes são CUMULATIVOS desde o attach: um processo pode
// diluir sua taxa (slow-burn) mas não pode apagar bytes já contabilizados.
// Não equivale a cada bit físico na mídia (writeback/buffer), mas é a
// contabilidade que o kernel faz por processo — mesma superfície do
// /proc/<pid>/io usado na validação Colab.
#[cfg_attr(feature = "user", derive(serde::Serialize, Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PhiBitFlowEntry {
    pub write_bytes: u64,
    pub write_calls: u64,
    pub read_bytes: u64,
    pub read_calls: u64,
    pub last_seen_ns: u64,
}

#[cfg(feature = "user")]
unsafe impl aya::Pod for PhiBitFlowEntry {}
