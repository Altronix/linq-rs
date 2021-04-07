set(LIBUSB_VERSION "1.0.24")
set(LIBUSB_NAME "libusb-${LIBUSB_VERSION}")
set(LIBUSB_SRC "${EXTERNAL_DIR}/${LIBUSB_NAME}")
set(LIBUSB_DST "${EXTERNAL_DIR}")
set(LIBUSB_TAR "${DOWNLOAD_DIR}/${LIBUSB_NAME}.tar.gz")
set(LIBUSB_TEST_FILE "${LIBUSB_SRC}/README")
if(MSVC)
  include("${WORKSPACE_ROOT}/cmake/Modules/FindLibusbHelpersWin.cmake")
else()
  include("${WORKSPACE_ROOT}/cmake/Modules/FindLibusbHelpersUnix.cmake")
endif()