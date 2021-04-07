#include <signal.h>
#include <stdio.h>

#include "linq.h"

const uint8_t g_preamble[64] = {
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
    0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1
};

void
ctrlc(int dummy)
{
    // netw_stop();
}

void
callback(const char* type, const char* data)
{
    printf("type: %s\n", type);
    printf("data: %s\n", data);
}

void
write_ack(usbh_s* usb)
{
    char ack[64];
    memset(ack, 0, sizeof(ack));
    snprintf(ack, sizeof(ack), "ACK");
    usbh_send(usb, "0", (unsigned char*)ack, 64);
}

int
main(int argc, char* argv[])
{
    signal(SIGINT, ctrlc);
    printf("Version: %s\n", usbh_version());
    uint32_t sz;
    char buffer[1024];
    char* mem;

    usbh_s* usbh = usbh_create();
    assert_fn(usbh);

    usbh_add_product(usbh, LINQ_TRANSPORT_USB_HID, 0x0461, 0x0020);
    usbh_add_product(usbh, LINQ_TRANSPORT_USB_CDC, 0x3333, 0x4444);
    usbh_scan(usbh);

    mem = usbh_summary_alloc(usbh);
    printf("%s\n", mem);
    usbh_summary_free(&mem);

    // Write preamble
    usbh_send(usbh, "0", g_preamble, 64);

    // Recv Ack
    sz = 64;
    usbh_recv(usbh, "0", (uint8_t*)buffer, &sz, 2000);
    printf("%.*s\n", sz, buffer);

    // Write length
    sz = snprintf(NULL, 0, "GET /ATX/about");
    mem = (char*)&sz;
    memset(buffer, 0, sizeof(buffer));
    buffer[0] = mem[0];
    buffer[1] = mem[1];
    usbh_send(usbh, "0", (uint8_t*)buffer, 64);

    // Recv ack
    sz = 64;
    usbh_recv(usbh, "0", (uint8_t*)buffer, &sz, 2000);
    printf("%.*s\n", sz, buffer);

    // Send request
    memset(buffer, 0, sizeof(buffer));
    sz = snprintf(buffer, sizeof(buffer), "GET /ATX/about");
    usbh_send(usbh, "0", (uint8_t*)buffer, 64);

    // Recv Prealmble
    sz = 64;
    usbh_recv(usbh, "0", (uint8_t*)buffer, &sz, 2000);
    if (!memcmp(buffer, g_preamble, sizeof(g_preamble))) {
        // Received start of message
        printf("Received preamble\n");
        write_ack(usbh);

        // Received length
        usbh_recv(usbh, "0", (uint8_t*)buffer, &sz, 2000);
        uint32_t i = 0, n = *((uint16_t*)buffer);
        printf("Byte[0] %x\n", (uint8_t)buffer[0]);
        printf("Byte[1] %x\n", (uint8_t)buffer[1]);
        printf("Received %d bytes\n", n);
        n = n % 64 == 0 ? n / 64 : (n / 64) + 1;
        printf("Expect %d packets\n", n);

        // Read in payload
        while (i < n) {
            // Write ack for length + packets
            write_ack(usbh);
            sz = 64;
            usbh_recv(usbh, "0", (uint8_t*)&buffer[64 * i], &sz, 2000);
            i++;
        }
    } else {
        printf("Not supported\n");
    }

    printf("Received %s\n", buffer);
    printf("Shutting down...\n");
    usbh_destroy(&usbh);
}

