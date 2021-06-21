#include <CoreFoundation/CoreFoundation.h>

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct {
    int32_t status;
    CFStringRef msg;
} FFIVoidResult;

typedef struct {
    CFStringRef address;
    CFStringRef status;
    CFStringRef holdings;
    CFStringRef rewards;
    CFStringRef pending_rewards;
} FFIAccountViewData;

typedef struct {
    int32_t status;
    CFStringRef error;
    FFIAccountViewData data;
} FFIGetInfosResult;

FFIVoidResult ffi_bootstrap();
FFIGetInfosResult ffi_get_infos(const char *address);
