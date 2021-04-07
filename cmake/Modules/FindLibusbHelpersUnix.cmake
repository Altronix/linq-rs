#
# Find LibUSB
#
function (find_libusb result)
  find_library(${result}
    NAMES 
      libusb-1.0.a
      libusb-1.0.lib
    NO_CMAKE_SYSTEM_PATH
    PATHS "${CMAKE_INSTALL_PREFIX}"
    PATH_SUFFIXES "lib")
endfunction()

#
# Import Libusb
#
function (import_libusb lib inc)
  add_library(libusb STATIC IMPORTED)
  set_target_properties(libusb PROPERTIES
    IMPORTED_LOCATION "${lib}"
    INTERFACE_INCLUDE_DIRECTORIES "${inc}")
endfunction()

#
# Build LibUSB
#
function (build_libusb)
  message(STATUS "Checking LIBUSB Extract...")
  check_extract(
    "${LIBUSB_TAR}" 
    "${LIBUSB_DST}" 
    "${LIBUSB_TEST_FILE}"
    LIBUSB_EXTRACT_RESULT)
  message(STATUS "LIBUSB_EXTRACT_RESULT: ${LIBUSB_EXTRACT_RESULT}")

  # ./autogen
  message(STATUS "Autogen Libusb")
  execute_process(
    COMMAND ./autogen.sh
    RESULT_VARIABLE LIBUSB_AUTOGEN_RESULT
    WORKING_DIRECTORY "${LIBUSB_SRC}")
  message(STATUS "LIBUSB_AUTOGEN_RESULT: ${LIBUSB_AUTOGEN_RESULT}")

  # ./configure
  message(STATUS "Configure Libusb")
  execute_process(
    COMMAND ./configure --prefix=${CMAKE_INSTALL_PREFIX} --with-pic
    RESULT_VARIABLE LIBUSB_CONFIGURE_RESULT
    WORKING_DIRECTORY "${LIBUSB_SRC}")
  message(STATUS "LIBUSB_CONFIGURE_RESULT: ${LIBUSB_CONFIGURE_RESULT}")

  # make
  message(STATUS "Build Libusb")
  execute_process(
    COMMAND make
    RESULT_VARIABLE LIBUSB_MAKE_RESULT
    WORKING_DIRECTORY "${LIBUSB_SRC}")
  message(STATUS "LIBUSB_MAKE_RESULT: ${LIBUSB_MAKE_RESULT}")

  # make install
  message(STATUS "Install Libusb")
  execute_process(
    COMMAND make install
    RESULT_VARIABLE LIBUSB_INSTALL_RESULT
    WORKING_DIRECTORY "${LIBUSB_SRC}")
  message(STATUS "LIBUSB_INSTALL_RESULT: ${LIBUSB_INSTALL_RESULT}")
endfunction()
