# Event Tracing for Windows ETW Bypass in Rust

I have written about this on my blog, check it [here](https://fluxsec.red/etw-patching-rust)!

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

I use my crate [export-resolver](https://crates.io/crates/export-resolver) to perform resolve the function virtual addresses 
dynamically via the PEB.

## Proof

**Before** patching:

![image](https://github.com/0xflux/ETW-Bypass-Rust/assets/49762827/fb29813c-0dd5-42c7-bf65-a9c28527651a)

<img width="1481" alt="etw-test-1" src="https://github.com/0xflux/ETW-Bypass-Rust/assets/49762827/9a7e8f70-5a03-45eb-b5df-53d521693778">


**After** patching:

![image](https://github.com/0xflux/ETW-Bypass-Rust/assets/49762827/9fe3a86b-fdd5-41e4-aad6-7faa997abf97)

<img width="1259" alt="etw-test-two" src="https://github.com/0xflux/ETW-Bypass-Rust/assets/49762827/3365bd7a-21d6-4b4c-98c9-283eb25ef6da">

# LEGAL DISCLAIMER 

This project, including all associated source code and documentation, is developed and shared solely for educational, research, and defensive purposes in the field of cybersecurity. It is intended to be used exclusively by cybersecurity professionals, researchers, and educators to enhance understanding, develop defensive strategies, and improve security postures.

Under no circumstances shall this project be used for criminal, unethical, or any other unauthorized activities. This is meant to serve as a resource for learning and should not be employed for offensive operations or actions that infringe upon any individual's or organization's rights or privacy.

The author of this project disclaims any responsibility for misuse or illegal application of the material provided herein. By accessing, studying, or using this project, you acknowledge and agree to use the information contained within strictly for lawful purposes and in a manner that is consistent with ethical guidelines and applicable laws and regulations.

USE AT YOUR OWN RISK. If you decide to use this software CONDUCT A THOROUGH INDEPENDENT CODE REVIEW to ensure it meets your standards. No unofficial third party dependencies are included to minimise attack surface of a supply chain risk. I cannot be held responsible for any problems that arise as a result of executing this, the burden is on the user of the software to validate its safety & integrity. All care has been taken to write safe code.

It is the user's responsibility to comply with all relevant local, state, national, and international laws and regulations related to cybersecurity and the use of such tools and information. If you are unsure about the legal implications of using or studying the material provided in this project, please consult with a legal professional before proceeding. Remember, responsible and ethical behavior is paramount in cybersecurity research and practice. The knowledge and tools shared in this project are provided in good faith to contribute positively to the cybersecurity community, and I trust they will be used with the utmost integrity.
