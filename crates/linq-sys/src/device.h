#ifndef DEVICE_H
#define DEVICE_H

#include "containers.h"

#ifdef __cplusplus
extern "C"
{
#endif

    typedef struct device_s
    {
        char sid[LINQ_SID_LEN];
        char pid[LINQ_PID_LEN];
        linq_send_fn send;
        void (*free_fn)(struct device_s*);
    } device_s;

    static void device_free_fn(device_s** device_p)
    {
        device_s* d = *device_p;
        *device_p = NULL;
        d->free_fn(d);
        free_fn(d);
    }

    MAP_INIT_STR(device, device_s, device_free_fn);

#ifdef __cplusplus
}
#endif
#endif
