# I/O Models

## Intro
In our programming task we do everyday, there is a lot of I/O calls (open file, database calls, request to other endpoint..). But since those calls are very well-abstracted by our programming language, we usually don't know about what happens underhood.
An I/O Model is a mechanism for a program in userspace to get the data it wanted (file on disk, network inteface, ...) through communicating with the OS (kernelspace).

## Userspace vs Kernelspace
### Kernel Space
Kernel space is the reserved memory that only allows OS's kernel to access. When a kernel directed a device to access its data (e.g., Tell a disk to read some specific blocks), the device's driver will copy the requested data on to a buffer on kernel space.
### User Space
In contrast, User space is the memory region of our program, and since we can't directly control the hardware, we must ask the OS's kernel to do the controlling for us. I/O model is about the way our program asking kernel to control peripheral devices.

## Unix I/O Models
In this series we will focus on available I/O Models, how does it work, where it is used.
There are two distinc phases for an I/O Process:

1. Kernel instructs the hardware's driver and wait for it to copy the data on to a buffer in kernel space.
2. When the data is ready, Kernel will copy the data from kernel space's buffer in to our process's buffer.

### Blocking I/O
Blocking I/O is the most commonly used method of I/O. In this model, when our program want to do some I/O operations, it will instruct the kernel to do that operation and block the execution of calling process until the operation has completed.
![Blocking I/O](https://notes.shichao.io/unp/figure_6.1.png)

This method is easy to use, but it has the drawback of Only one I/O operation can be processed at one time. In order to do more operations, this method is usually be used in conjunction of multi-threading.

### Nonblocking I/O
Instead of letting the kernel suspend the execution of our process (blocking), we could instruct the system call as non-blocking, so that it returns immediately if the data is not ready on kernel space yet. 
(It's still block when the data is transfering from kernel space to user space)

![Nonblocking I/O](https://notes.shichao.io/unp/figure_6.2.png)

When a process sitting and keep checking for an I/O call's status, it's **polling**.

### I/O Multiplexing
Instead of just waiting for one IO call (the system call it file descriptor) to finish, there is system calls `select` or `poll` allow us to wait for multiple descriptor at the same time. The calling process will still be blocked until there's a ready descriptor in the waiting list.

![Multiplexing I/O](https://notes.shichao.io/unp/figure_6.3.png)

### More Coming ...

