const CORE_MODULES: [&str; 27] = [
    "assert",
    "buffer",
    "child_process",
    "console",
    "constants",
    "crypto",
    "dgram",
    "dns",
    "events",
    "fs",
    "http",
    "https",
    "module",
    "net",
    "os",
    "path",
    "querystring",
    "readline",
    "repl",
    "stream",
    "string_decoder",
    "timers",
    "tls",
    "tty",
    "url",
    "util",
    "vm",
];

pub fn is_core_module(module: &str) -> bool {
    CORE_MODULES.contains(&module)
}
