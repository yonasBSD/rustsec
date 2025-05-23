//! Documentation comments that go on enum variants.

pub(crate) const COMMENTS: &[(&str, &str)] = &[
// OS
("android", "Google's Android mobile operating system"),
("cuda", "CUDA parallel computing platform"),
("dragonfly", "DragonflyBSD"),
("emscripten", "The emscripten JavaScript transpiler"),
("freebsd", "The FreeBSD operating system"),
("fuchsia", "Google's next-gen Rust OS"),
("haiku", "Haiku, an open source BeOS clone"),
("hermit", "HermitCore is a novel unikernel operating system targeting a scalable and predictable runtime behavior for HPC and cloud environments"),
("illumos", "illumos is a partly free and open-source Unix operating system based on OpenSolaris"),
("ios", "Apple's iOS mobile operating system"),
("linux", "Linux"),
("macos", "Apple's Mac OS X"),
("netbsd", "The NetBSD operating system"),
("openbsd", "The OpenBSD operating system"),
("redox", "Redox, a Unix-like OS written in Rust"),
("solaris", "Oracle's (formerly Sun) Solaris operating system"),
("tvOS", "AppleTV operating system"),
("wasi", "The WebAssembly System Interface"),
("windows", "Microsoft's Windows operating system"),
("vxworks", "VxWorks is a deterministic, priority-based preemptive RTOS with low latency and minimal jitter"),
// Arch
("aarch64", "ARMv8 64-bit architecture"),
("arm", "32-bit ARM architecture"),
("asm", "asm.js output"),
("mips", "32-bit MIPS CPU architecture"),
("mips64", "64-bit MIPS CPU architecture"),
("msp430", "16-bit MSP430 microcontrollers"),
("nvptx64", "64-bit NVIDIA PTX"),
("powerpc", "32-bit POWERPC platform"),
("powerpc64", "64-bit POWERPC platform"),
("riscv", "RISC-V CPU architecture"),
("s390x", "64-bit IBM z/Architecture"),
("sparc", "32-bit SPARC CPU architecture"),
("sparc64", "64-bit SPARC CPU architecture"),
("thumbv6", "16-bit ARM CPU architecture subset"),
("thumbv7", "16-bit ARM CPU architecture subset"),
("wasm32", "Web Assembly (32-bit)"),
("x86", "Generic x86 CPU architecture"),
("x86_64", "'AMD64' CPU architecture"),
// Env
("", "None"),
("gnu", "The GNU C Library (glibc)"),
("msvc", "Microsoft Visual C(++)"),
("musl", "Clean, efficient, standards-conformant libc implementation."),
("sgx", "Intel Software Guard Extensions (SGX) Enclave"),
("uclibc", "C library for developing embedded Linux systems"),
];
