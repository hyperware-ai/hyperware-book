# Startup, Spindown, and Crashes

Along with learning how processes communicate, understanding the lifecycle paradigm of Kinode processes is essential to developing useful p2p applications.
Recall that a 'package' is a userspace construction that contains one or more processes.
The Kinode kernel is only aware of processes.
When a process is first initialized, its compiled Wasm code is loaded into memory and, if the code is valid, the process is added to the kernel's process table.
Then, the kernel starts the process by calling the `init` function (which is common to all processes).

This scenario is identical to when a process is re-initialized after being shut down. From the perspective of both the kernel and the process code, there is no difference.

## Persisting State With Processes

Given that Kinodes can, comporting with the realities of the physical world, be turned off, a well-written process must withstand being shut down and re-initialized at any time.
This raises the question: how does a process persist information between initializations?
There are two ways: either the process can use the built-in `set_state` and `get_state` functions, or it can send data to a process that does this for them.

The first option is a maximally-simple way to write some bytes to disk (where they'll be backed up, if the node owner has configured that behavior).
The second option is vastly more general, because runtime modules, which can be messaged directly from custom userspace processes, offer any number of APIs.
So far, there are three modules built into Kinode OS that are designed for persisted data: a [filesystem](../files.md), a [key-value store, and a SQLite database](../databases.md).

Each of these modules offer APIs accessed via message-passing and write data to disk.
Between initializations of a process, this data remains saved, even backed up.
The process can then retrieve this data when it is re-initialized.