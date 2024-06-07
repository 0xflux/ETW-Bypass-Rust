# Event Tracing for Windows ETW Bypass in Rust

Event Tracing for Windows (ETW) is a logging framework provided by Microsoft for Windows. 
ETW allows software developers and system admins to obtain detailed, real-time 
diagnostic information about system and application behavior. It provides a unified and efficient mechanism 
for logging system events, tracking application performance, debugging, and monitoring security. ETW operates 
by defining trace providers, which emit events, and trace sessions, which collect and store these events for 
analysis. 

Endpoint Detection and Response (EDR) solutions leverage ETW to monitor and analyse system events in 
real-time, enabling the detection and investigation of malicious activities and security threats 
on endpoints.

ETW can be bypassed in user mode by modifying the address of the `NtTraceEvent` 
function in `ntdll.dll`. This involves patching the syscall stub of `NtTraceEvent` to change its implementation 
to a `ret` (return) instruction. By doing so, any attempts to invoke ETW functionality from user mode are 
effectively thwarted because the function will immediately return without performing any logging actions.

ETW bypasses are one method of EDR bypassing and EDR evasion.

For a little fun, I have implemented some of my Hell's Gate work into the library, and I'll probably 
make it fully Hell's Gate compliant in the future. I use my crate 
[export-resolver](https://crates.io/crates/export-resolver) to perform the Hell's Gate technique.

If you want to check out my blog post on Hell's Gate, click 
[here](https://fluxsec.red/rust-edr-evasion-hells-gate).

## Proof

**Before** patching:

![image](https://github.com/0xflux/ETW-Bypass-Rust/assets/49762827/fb29813c-0dd5-42c7-bf65-a9c28527651a)

**After** patching:

![image](https://github.com/0xflux/ETW-Bypass-Rust/assets/49762827/9fe3a86b-fdd5-41e4-aad6-7faa997abf97)
