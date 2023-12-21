#define _AMD64_
#include <wdm.h>

NTSTATUS DriverEntry(void* a, void* b) {
    DbgPrint("Hello from Connors Driver!");
    return STATUS_SUCCESS;
}