#ifndef LINQ_H_
#define LINQ_H_

// clang-format off
#if defined _WIN32
#  include <assert.h>
#  include <stdarg.h>
#  include <stdbool.h>
#  include <stdint.h>
#  include <stdio.h>
#  include <string.h>
#  if defined LINQ_STATIC
#    define LINQ_EXPORT
#  elif defined DLL_EXPORT
#    define LINQ_EXPORT __declspec(dllexport)
#  else
#    define LINQ_EXPORT __declspec(dllimport)
#  endif
#else
#  include <assert.h>
#  include <stdarg.h>
#  include <stdbool.h>
#  include <stddef.h>
#  include <stdint.h>
#  include <stdio.h>
#  include <stdlib.h>
#  include <string.h>
#  include <sys/stat.h>
#  include <sys/types.h>
#  include <unistd.h>
#  define LINQ_EXPORT
#endif
// clang-format on

#define assert_fn assert
#define malloc_fn malloc
#define free_fn free

#ifndef LINQ_SID_LEN
#define LINQ_SID_LEN 64
#endif

#ifndef LINQ_PID_LIN
#define LINQ_PID_LEN 64
#endif

#ifndef LOG_MESSAGE_LEN
#define LOG_MESSAGE_LEN 128
#endif

#ifdef __cplusplus
extern "C"
{
#endif

    typedef enum
    {
        LINQ_TRACE,
        LINQ_DEBUG,
        LINQ_INFO,
        LINQ_WARN,
        LINQ_ERROR,
        LINQ_FATAL,
        LINQ_NONE
    } E_LOG_LEVEL;

    typedef struct log_callback_s
    {
        void* context;
        uint32_t line;
        uint32_t tick;
        const char* file;
        const char* level;
        const char* category;
        char message[LOG_MESSAGE_LEN];
    } log_callback_s;
    typedef void (*log_callback_fn)(log_callback_s*);

    // Supported transports via this binding
    typedef enum E_LINQ_TRANSPORT
    {
        LINQ_TRANSPORT_USB_HID,
        LINQ_TRANSPORT_USB_CDC,
        LINQ_TRANSPORT_ZMTP,
    } E_LINQ_TRANSPORT;

    // Types of requests
    typedef enum E_LINQ_REQUEST_METHOD
    {
        LINQ_REQUEST_METHOD_RAW = 0,
        LINQ_REQUEST_METHOD_GET = 1,
        LINQ_REQUEST_METHOD_POST = 2,
        LINQ_REQUEST_METHOD_DELETE = 3
    } E_LINQ_REQUEST_METHOD;

    // An error code
    typedef enum
    {
        LINQ_ERROR_OK = 0,
        LINQ_ERROR_OOM = -1,
        LINQ_ERROR_BAD_ARGS = -2,
        LINQ_ERROR_PROTOCOL = -3,
        LINQ_ERROR_IO = -4,
        LINQ_ERROR_DEVICE_NOT_FOUND = -5,
        LINQ_ERROR_TIMEOUT = -6,
        LINQ_ERROR_SHUTTING_DOWN = -7,
        LINQ_ERROR_LIBUSB = -8,
        LINQ_ERROR_400 = 400,
        LINQ_ERROR_403 = 403,
        LINQ_ERROR_404 = 404,
        LINQ_ERROR_500 = 500,
        LINQ_ERROR_504 = 504,
    } E_LINQ_ERROR;

    // Callback for when sending a request is completed
    typedef void (
        *linq_request_complete_fn)(void*, const char* serial, E_LINQ_ERROR e);

    // Definition of a transmit method
    typedef void (*linq_send_fn)(
        E_LINQ_REQUEST_METHOD method,
        const char* path,
        uint32_t plen,
        const char* json,
        uint32_t jlen,
        linq_request_complete_fn,
        void*);

    // Forward declare main class
    typedef struct usbh_s usbh_s;
    typedef struct zmtp_s zmtp_s;

    // get version of this library
    LINQ_EXPORT const char* usbh_version();

    // get human readable error
    LINQ_EXPORT const char* usbh_strerror(E_LINQ_ERROR);

    // redirect log output
    LINQ_EXPORT void usbh_log_fn_set(log_callback_fn fn, void* ctx);

    // create a usbh instance
    LINQ_EXPORT usbh_s* usbh_create();

    // add product and device strings to monitor
    LINQ_EXPORT void
    usbh_add_product(usbh_s*, E_LINQ_TRANSPORT, uint32_t, uint32_t);

    // Free a usbh instance
    LINQ_EXPORT void usbh_destroy(usbh_s**);

    // Scan usb devices
    LINQ_EXPORT E_LINQ_ERROR usbh_scan(usbh_s* self);

    // Print summary of usbh devices
    LINQ_EXPORT char* usbh_summary_alloc(usbh_s* linq);

    // Free the memory after use
    LINQ_EXPORT void usbh_summary_free(char**);

    // Send a request to a device
    LINQ_EXPORT E_LINQ_ERROR usbh_send(
        usbh_s* usbh,
        const char* name,
        const uint8_t* bytes,
        uint32_t plen);

    // Recv a request from a device
    LINQ_EXPORT E_LINQ_ERROR usbh_recv(
        usbh_s* self,
        const char* name,
        uint8_t* bytes,
        uint32_t* sz,
        uint32_t timeout);
    // Poll network
#ifdef __cplusplus
}
#endif
#endif /* LINQ_H_ */
