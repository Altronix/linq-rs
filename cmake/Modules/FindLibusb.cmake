include ("${WORKSPACE_ROOT}/cmake/Modules/FindLibusbHelpers.cmake")

find_libusb(Libusb_LIBRARIES)
    
# Find libusb.a
if("${Libusb_LIBRARIES}" STREQUAL "Libusb_LIBRARIES-NOTFOUND")
  message(STATUS "${Libusb_LIBRARIES}")
  message(STATUS "Building Libusb...")
  build_libusb()
  find_libusb(Libusb_LIBRARIES)
  if("${Libusb_LIBRARIES}" STREQUAL "Libusb_LIBRARIES-NOTFOUND")
    message(FATAL "Could not build Libusb!")
  endif()
else()
  message(STATUS "LIBUSB_LOC: ${Libusb_LIBRARIES}")
endif()


# Set header loc
set(Libusb_INCLUDE_DIRS "${CMAKE_INSTALL_PREFIX}/include")
import_libusb("${Libusb_LIBRARIES}" "${Libusb_INCLUDE_DIRS}")

include(FindPackageHandleStandardArgs)
find_package_handle_standard_args(Libusb
  FOUND_VAR Libusb_FOUND
  REQUIRED_VARS Libusb_LIBRARIES Libusb_INCLUDE_DIRS)
