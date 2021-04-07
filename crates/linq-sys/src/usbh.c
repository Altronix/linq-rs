#include "containers.h"
#include "device.h"
#include "errno.h"
#include "linq.h"
#include "log.h"

#include "libusb-1.0/libusb.h"

#define usb_info(...) log_info("USB", __VA_ARGS__)
#define usb_warn(...) log_warn("USB", __VA_ARGS__)
#define usb_debug(...) log_debug("USB", __VA_ARGS__)
#define usb_trace(...) log_trace("USB", __VA_ARGS__)
#define usb_error(...) log_error("USB", __VA_ARGS__)
#define usb_fatal(...) log_fatal("USB", __VA_ARGS__)

#define IN 1 | LIBUSB_ENDPOINT_IN
#define OUT 2 | LIBUSB_ENDPOINT_OUT

// TODO we should have proper Vendor and product ID's.
#define K64_VID 0x0461
#define K64_PID 0x0020
#define M5_VID 0x3333
#define M5_PID 0x4444

#define USBH_DEVICE_SUMMARY_FORMAT                                             \
    "{"                                                                        \
    "\"vendor\":%d,"                                                           \
    "\"product\":%d,"                                                          \
    "\"serial\":\"%s\""                                                        \
    "}"

static int eusb = 0;

typedef libusb_context usb_context;

// usb device inherits device
typedef struct usbh_device_s
{
    device_s meta;                // Must be on top
    libusb_device_handle* handle; //
    libusb_device* device;        //
    uint8_t serial;
    uint16_t product;
    uint16_t vendor;
} usbh_device_s;

// Supported product types
typedef struct product_s
{
    E_LINQ_TRANSPORT transport;
    uint16_t product;
    uint16_t vendor;
} product_s;

static void
usbh_device_free_fn(device_s* d)
{
    // Hash map is about to free device, we clean up ourself
    usbh_device_s* device = (usbh_device_s*)d;
    if (device->handle) libusb_close(device->handle);
}

MAP_INIT_INT_W_FREE(product, product_s);

typedef struct usbh_s
{
    usb_context* context;
    product_map_s* products;
    device_map_s* devices;
} usbh_s;

static void
transport_install_hid(
    usbh_s* self,
    usbh_device_s* dev,
    product_s* prod,
    struct libusb_device_descriptor* desc)
{
    usb_trace("hid install: [%.4x/%.4x]", desc->idVendor, desc->idProduct);
    int err;
    // TODO the HID doesn't support serial numbers so we hard code all hid's
    //      to a single serial. Which means we only support one at a time
    // err = libusb_get_string_descriptor_ascii(
    //     dev->handle,
    //     desc->iSerialNumber,
    //     (unsigned char*)dev->meta.sid,
    //     sizeof(dev->meta.sid));
    // if (err) usb_error("%s", libusb_strerror(err));
    dev->product = prod->product;
    dev->vendor = prod->vendor;
    dev->meta.free = usbh_device_free_fn;
    snprintf(dev->meta.pid, sizeof(dev->meta.pid), "%d", prod->product);
    snprintf(dev->meta.sid, sizeof(dev->meta.sid), "%d", 0);
    // TODO populate transmitter

    for (int i = 0; i < 1; i++) {
        if (libusb_kernel_driver_active(dev->handle, i)) {
            // NOTE detatching only required on unix. But seems harmless error 
            //      on windows
            usb_info("detatching kernel driver [%d]", i);
            err = libusb_detach_kernel_driver(dev->handle, i);
            if (err) usb_error("[%s]", libusb_strerror(err));
        }
        usb_info("claiming interface [%d]", i);
        err = libusb_claim_interface(dev->handle, i);
        if (err) usb_error("[%s]", libusb_strerror(err));
    }
}

LINQ_EXPORT const char*
usbh_version()
{
#ifdef LINQ_VERSION
    static const char* version = LINQ_VERSION;
#else
    static const char* version = "undefined";
#endif
    return version;
}

// get human readable error
LINQ_EXPORT const char*
usbh_strerror(E_LINQ_ERROR e)
{
    // TODO this should be more easier macro that maps nicer with ENUM
    static const char* unknown = "Unknown error";
    static const char* errors[] = { "OK",
                                    "Out of memory",
                                    "Bad arguments",
                                    "Protocol error",
                                    "Internal IO error",
                                    "Device not found",
                                    "Device Timeout",
                                    "Shutting down" };
    const char* result = unknown;
    int test = e * -1;
    switch ((test)) {
        case 0:
        case 1:
        case 2:
        case 3:
        case 4:
        case 5:
        case 6:
        case 7: result = errors[-e]; break;
        case 8: result = libusb_strerror(eusb); break;
        default: result = unknown;
    }
    return result;
}

LINQ_EXPORT void
usbh_log_fn_set(log_callback_fn fn, void* ctx)
{
    log_set_callback_fn(fn, ctx);
}

LINQ_EXPORT usbh_s*
usbh_create()
{
    int err;
    usbh_s* self = malloc_fn(sizeof(usbh_s));
    if (self) {
        memset(self, 0, sizeof(usbh_s));
        err = libusb_init(&self->context);
        if (!(err == 0)) usb_fatal("Failed to initialize libusb!");
        self->devices = device_map_create();
        if (!self->devices) usb_fatal("Failed to allocated hash map devices!");
        self->products = product_map_create();
        if (!self->products) usb_fatal("Failed to allocate hash map products!");
        usbh_add_product(self, LINQ_TRANSPORT_USB_HID, K64_VID, K64_PID);
        usbh_add_product(self, LINQ_TRANSPORT_USB_CDC, M5_VID, M5_PID);
    }
    return self;
}

LINQ_EXPORT
void
usbh_destroy(usbh_s** self_p)
{
    usbh_s* self = *self_p;
    *self_p = NULL;
    self->context = NULL;
    product_map_destroy(&self->products);
    device_map_destroy(&self->devices);
    libusb_exit(self->context);
    free_fn(self);
}

LINQ_EXPORT
void
usbh_add_product(
    usbh_s* self,
    E_LINQ_TRANSPORT transport,
    uint32_t vendor,
    uint32_t product)

{
    product_s* p = malloc_fn(sizeof(product_s));
    if (!p) usb_fatal("Failed to allocate memory");
    if (!(transport == LINQ_TRANSPORT_USB_HID ||
          transport == LINQ_TRANSPORT_USB_CDC)) {
        usb_fatal("Failed to install proper USB transport!");
    }
    memset(p, 0, sizeof(product_s));
    p->transport = transport;
    p->vendor = vendor;
    p->product = product;
    product_map_add(self->products, p->product, &p);
}

static int
scan_process_product(
    usbh_s* self,
    libusb_device* libusb_dev,
    struct libusb_device_descriptor* desc,
    product_s* product)
{
    int err = LINQ_ERROR_OK;
    usbh_device_s* device;
    usb_info("scan match [%.4x/%.4x]", product->vendor, product->product);
    if (LINQ_TRANSPORT_USB_CDC == product->transport) {
        usb_fatal("CDC not implemented yet!");
    } else {
        device = malloc_fn(sizeof(usbh_device_s));
        if (!device) usb_fatal("Out of memory!");
        memset(device, 0, sizeof(device_s));
        err = libusb_open(libusb_dev, &device->handle);
        if (err) {
            free_fn(device);
            usb_error("%s", libusb_strerror(err));
            eusb = err;
            err = LINQ_ERROR_LIBUSB;
        } else {
            transport_install_hid(self, device, product, desc);
            if (device_map_get(self->devices, device->meta.sid)) {
                usb_info("Device already exists...");
                usb_warn("Note this driver only supports 1 device "
                         "at a time because serial number strategy "
                         "of our products is not compat with USB");
                libusb_close(device->handle);
                free_fn(device);
            } else {
                usb_info("Adding device");
                device_map_add(
                    self->devices, device->meta.sid, (device_s**)&device);
            }
        }
    }
    return err;
}

static int
scan_process_descriptor(
    usbh_s* self,
    libusb_device* libusb_dev,
    struct libusb_device_descriptor* desc)
{
    int err = LINQ_ERROR_OK;
    map_iter iter;
    product_s* product;
    map_foreach(self->products, iter)
    {
        if (map_has_key(self->products, iter) &&
            (product = map_val(self->products, iter))) {
            if (desc->idVendor == product->vendor &&
                desc->idProduct == product->product) {
                err = scan_process_product(self, libusb_dev, desc, product);
                if (err) break;
            }
        }
    }
    return err;
}

LINQ_EXPORT E_LINQ_ERROR
usbh_scan(usbh_s* self)
{
    usb_trace("%s", "usbh_scan()");
    libusb_device **devs, *libusb_dev;
    const char* serial;
    int n = 0, i = 0, err;
    uint32_t count = libusb_get_device_list(self->context, &devs);
    if (count > 0) {
        libusb_dev = devs[i];
        while (libusb_dev) {
            struct libusb_device_descriptor desc;
            err = libusb_get_device_descriptor(libusb_dev, &desc);
            if (!(err == 0)) {
                usb_error("usb device descriptor error!");
                eusb = err;
                err = LINQ_ERROR_LIBUSB;
                break;
            }
            usb_debug("scan found [%.4x/%.4x]", desc.idVendor, desc.idProduct);

            err = scan_process_descriptor(self, libusb_dev, &desc);
            if (err) break;

            libusb_dev = devs[++i];
        }

        libusb_free_device_list(devs, 1);
    }

    return err ? err : n;
}

LINQ_EXPORT char*
usbh_summary_alloc(usbh_s* linq)
{
    char* alloc = NULL;
    device_map_s* map = linq->devices;
    uint32_t n = device_map_size(map), spot = 0, l = (n + 1) * 128;
    alloc = malloc_fn(l);
    if (alloc) {
        alloc[spot++] = '[';
        map_iter i;
        device_s* node;
        map_foreach(map, i)
        {
            if ((map_has_key(map, i) && (node = map_val(map, i)))) {
                usbh_device_s* d = (usbh_device_s*)node;
                spot += snprintf(
                    &alloc[spot],
                    l - spot,
                    USBH_DEVICE_SUMMARY_FORMAT,
                    d->vendor,
                    d->product,
                    d->meta.sid);
                if (--n) {
                    if (spot < l) alloc[(spot++)] = ',';
                }
            }
        }
        if (spot < l) alloc[(spot)++] = ']';
        if (spot < l) alloc[(spot)++] = '\0';
    }
    return alloc;
}

LINQ_EXPORT void
usbh_summary_free(char** mem_p)
{
    char* mem = *mem_p;
    *mem_p = NULL;
    free_fn(mem);
}

LINQ_EXPORT E_LINQ_ERROR
usbh_send(usbh_s* linq, const char* name, const uint8_t* b, uint32_t len)
{
    int txed = 0, err;
    uint8_t* bytes = (uint8_t*)b; // erase const for libusb :(
    device_s* d = *device_map_get(linq->devices, name);
    usbh_device_s* device = (usbh_device_s*)d;
    if (!device) log_fatal("Device not found! [%s]", name);
    err = libusb_bulk_transfer(device->handle, OUT, bytes, len, &txed, 0);
    if (err < 0) {
        eusb = err;
        usb_error("Transmit fail [%s]", libusb_strerror(err));
        return LINQ_ERROR_LIBUSB;
    } else if (err > 0 && !(txed == len)) {
        usb_fatal("TODO sync transfer incomplete!");
        return LINQ_ERROR_LIBUSB;
    } else {
        return LINQ_ERROR_OK;
    }
}

LINQ_EXPORT E_LINQ_ERROR
usbh_recv(
    usbh_s* self,
    const char* name,
    uint8_t* bytes,
    uint32_t* sz,
    uint32_t timeout)
{
    int txed = 0, err;
    device_s* d = *device_map_get(self->devices, name);
    usbh_device_s* device = (usbh_device_s*)d;
    if (!device) log_fatal("Device not found! [%s]", name);

    err = libusb_bulk_transfer(device->handle, IN, bytes, *sz, &txed, timeout);
    if (err < 0) {
        eusb = err;
        usb_error("[%s] [%s]", libusb_strerror(err), strerror(errno));
        return LINQ_ERROR_LIBUSB;
    } else {
        usb_trace("rx [%d/%d]", txed, *sz);
        *sz = txed;
        // TODO CDC uses /r/n delimiter, HID has fixed receives
        return LINQ_ERROR_OK;
    }
}
