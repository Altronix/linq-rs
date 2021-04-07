set(LIBUSB_SOURCE_DIR ${EXTERNAL_DIR}/libusb-1.0.24)
set(LIBUSB_TEST_FILE ${LIBUSB_SOURCE_DIR}/README)
check_extract("${DOWNLOAD_DIR}/libusb-1.0.24.tar.gz" "${LIBUSB_TEST_FILE}")

if(NOT MSVC)
  # IE if(UNIX OR CYGWIN OR MINGW)
  ExternalProject_Add(libusb-project
    SOURCE_DIR ${LIBUSB_SOURCE_DIR}
    INSTALL_DIR ${CMAKE_INSTALL_PREFIX}
    BUILD_IN_SOURCE 1
    UPDATE_COMMAND ""
    CONFIGURE_COMMAND ./autogen.sh
    COMMAND ./configure --prefix=<INSTALL_DIR> --with-pic
    BUILD_COMMAND make
    INSTALL_COMMAND make install
    EXCLUDE_FROM_ALL true)
  # BUILD STATIC
  set(LIBUSB_LIBRARY libusb-1.0${CMAKE_STATIC_LIBRARY_SUFFIX})
  set(LIBUSB_LIBRARY ${CMAKE_INSTALL_PREFIX}/lib/${LIBUSB_LIBRARY})
  add_library(libusb-static STATIC IMPORTED)
  add_dependencies(libusb-static libusb-project)
  set_target_properties(libusb-static PROPERTIES
    IMPORTED_LOCATION ${LIBUSB_LIBRARY}
    INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_INSTALL_PREFIX}/include)
  # BUILD SHARED
  set(LIBUSB_LIBRARY libusb-1.0${CMAKE_SHARED_LIBRARY_SUFFIX})
  set(LIBUSB_LIBRARY ${CMAKE_INSTALL_PREFIX}/lib/${LIBUSB_LIBRARY})
  add_library(libusb-shared SHARED IMPORTED)
  add_dependencies(libusb-shared libusb-project)
  set_target_properties(libusb-shared PROPERTIES
    IMPORTED_LOCATION ${LIBUSB_LIBRARY}
    INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_INSTALL_PREFIX}/include)
else()
  file(MAKE_DIRECTORY ${CMAKE_INSTALL_PREFIX}/include/libusb-1.0)
  # BUILD_STATIC
  set(LIBUSB_LIBRARY libusb-1.0${CMAKE_STATIC_LIBRARY_SUFFIX})
  set(LIBUSB_LIBRARY ${CMAKE_INSTALL_PREFIX}/lib/${LIBUSB_LIBRARY})
  add_custom_target(libusb-project DEPENDS ${LIBUSB_LIBRARY})
  add_custom_command( 
      OUTPUT "${LIBUSB_LIBRARY}"
      COMMAND MSBuild.exe
              -p:Configuration=Release
              -p:Platform=x64
              ${LIBUSB_SOURCE_DIR}/msvc/libusb_static_2017.vcxproj
      COMMAND ${CMAKE_COMMAND}
              -E
              copy
              "${LIBUSB_SOURCE_DIR}/x64/Release/lib/libusb-1.0.lib"
              ${LIBUSB_LIBRARY}
      COMMAND ${CMAKE_COMMAND}
              -E
              copy
              ${LIBUSB_SOURCE_DIR}/libusb/libusb.h
              ${CMAKE_INSTALL_PREFIX}/include/libusb-1.0/)
  add_library(libusb-static STATIC IMPORTED)
  add_dependencies(libusb-static libusb-project)
  set_target_properties(libusb-static PROPERTIES
    IMPORTED_CONFIGURATIONS Release
    IMPORTED_LOCATION_RELEASE ${LIBUSB_LIBRARY}
    INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_INSTALL_PREFIX}/include
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_RUNTIME_OUTPUT_DIRECTORY}")

  # BUILD_SHARED
  set(LIBUSB_LIBRARY libusb-1.0${CMAKE_SHARED_LIBRARY_SUFFIX})
  set(LIBUSB_LIBRARY ${CMAKE_INSTALL_PREFIX}/bin/${LIBUSB_LIBRARY})
  message(STATUS "libusb: ${LIBUSB_LIBRARY}")
  add_custom_target(libusb-shared-project DEPENDS ${LIBUSB_LIBRARY})
  add_custom_command( 
      OUTPUT "${LIBUSB_LIBRARY}"
      COMMAND MSBuild.exe
              -p:Configuration=Release
              -p:Platform=x64
              ${LIBUSB_SOURCE_DIR}/msvc/libusb_dll_2017.vcxproj
      COMMAND ${CMAKE_COMMAND}
              -E
              copy
              "${LIBUSB_SOURCE_DIR}/x64/Release/dll/libusb-1.0.dll"
              ${LIBUSB_LIBRARY}
      COMMAND ${CMAKE_COMMAND}
              -E
              copy
              ${LIBUSB_SOURCE_DIR}/libusb/libusb.h
              ${CMAKE_INSTALL_PREFIX}/include/libusb-1.0/)
  add_library(libusb-shared SHARED IMPORTED)
  add_dependencies(libusb-shared libusb-shared-project)
  set_target_properties(libusb-shared PROPERTIES
    IMPORTED_CONFIGURATIONS Release
    IMPORTED_LOCATION_RELEASE ${LIBUSB_LIBRARY}
    INTERFACE_INCLUDE_DIRECTORIES ${CMAKE_INSTALL_PREFIX}/include
    RUNTIME_OUTPUT_DIRECTORY "${CMAKE_RUNTIME_OUTPUT_DIRECTORY}")
endif()
