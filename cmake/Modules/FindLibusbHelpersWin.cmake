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

function (build_libusb)
  # Extract libusb
  message(STATUS "Checking LIBUSB Extract...")
  check_extract(
    "${LIBUSB_TAR}" 
    "${LIBUSB_DST}" 
    "${LIBUSB_TEST_FILE}"
    LIBUSB_EXTRACT_RESULT)
  message(STATUS "LIBUSB_EXTRACT_RESULT: ${LIBUSB_EXTRACT_RESULT}")

  # Build USB
  message(STATUS "Building Libusb")
  execute_process( 
      COMMAND MSBuild.exe
              -p:Configuration=Release
              -p:Platform=x64
              ${LIBUSB_SRC}/msvc/libusb_static_2019.vcxproj
      RESULT_VARIABLE LIBUSB_RESULT)
  message(STATUS "LIBUSB_RESULT: ${LIBUSB_RESULT}")

  # Copy lib into lib installation folder
  message(STATUS "installing library")
  execute_process(
      WORKING_DIRECTORY "${LIBUSB_SRC}"
      COMMAND ${CMAKE_COMMAND}
              -E
              copy
              "${LIBUSB_SRC}/x64/Release/lib/libusb-1.0.lib"
              "${CMAKE_INSTALL_PREFIX}/lib/libusb-1.0.lib"
      RESULT_VARIABLE LIBUSB_RESULT)
  message(STATUS "LIBUSB_RESULT: ${LIBUSB_RESULT}")

  # Copy header into header installation folder
  message(STATUS "installing header")
  file(MAKE_DIRECTORY "${CMAKE_INSTALL_PREFIX}/include/libusb-1.0")
  execute_process(
      WORKING_DIRECTORY "${LIBUSB_SRC}"
      COMMAND ${CMAKE_COMMAND}
              -E
              copy
              ${LIBUSB_SRC}/libusb/libusb.h
              ${CMAKE_INSTALL_PREFIX}/include/libusb-1.0/libusb.h)
  message(STATUS "LIBUSB_RESULT: ${LIBUSB_RESULT}")
endfunction()
