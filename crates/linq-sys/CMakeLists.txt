cmake_policy(SET CMP0091 NEW)
cmake_minimum_required(VERSION 3.15...3.16)
project(linq)

# Configure our versioning
set(V_MAJOR 0)
set(V_MINOR 0)
set(V_PATCH 0)
set(V_TWEAK rc0)

# Set WORKSPACE_ROOT
get_filename_component(WORKSPACE_ROOT "../../" ABSOLUTE)
set(DOWNLOAD_DIR "${WORKSPACE_ROOT}/dl")
set(EXTERNAL_DIR "${WORKSPACE_ROOT}/external")


# Compiler stuff
set(CMAKE_EXPORT_COMPILE_COMMANDS on)
set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_POSITION_INDEPENDENT_CODE ON)
set(CMAKE_MODULE_PATH "${WORKSPACE_ROOT}/cmake/Modules")
# NOTE required this line here to make linking on windows easier
set(CMAKE_MSVC_RUNTIME_LIBRARY "MultiThreaded$<$<CONFIG:Debug>:Debug>")

# Add some helper routines from our cmake Modules directory
include("${WORKSPACE_ROOT}/cmake/Modules/modules.cmake")

# some options
log_options(LOG_OPTIONS)
set("LINQ_LOG_LEVEL" "INFO" CACHE STRING "[${LOG_OPTIONS}]")
set("LINQ_BUILD_APPS" TRUE CACHE BOOL "[Build demo apps]")
set("LINQ_BUILD_DEBUG" FALSE CACHE BOOL "[Build with debug symbols]")

if(LINQ_BUILD_DEBUG)
  set(CMAKE_BUILD_TYPE Debug)
endif()

# Put our install prefix as absolute path, create version define, other misc...
normalizeInstallPrefix()
version_config(LINQ ${V_MAJOR} ${V_MINOR} ${V_PATCH} ${V_TWEAK})
log_config(LINQ "${LINQ_LOG_LEVEL}")

# Find (and or build) our dependencies
find_package(Threads REQUIRED)
find_package(Libusb REQUIRED)

# Build linq library
list(APPEND LIBS ${Libusb_LIBRARIES})
list(APPEND LIBS ${CMAKE_THREAD_LIBS_INIT})
list(APPEND INCS "${Libusb_INCLUDE_DIRS}")
list(APPEND DEFS "${LINQ_VERSION_DEFINITIONS}")
list(APPEND DEFS "${LINQ_LOG_DEFINITIONS}")
message(STATUS "${LIBS}")
if(NOT MSVC)
  list(APPEND LIBS rt m stdc++ uuid dl udev)
else()
endif()

if(LINQ_BUILD_APPS)
  add_subdirectory(apps)
endif()

add_library(linq-static 
  src/linq.h 
  src/zmtp.c 
  src/usbh.c 
  src/device.h 
  src/containers.h 
  src/log.c 
  src/log.h)
target_link_libraries(linq-static ${LIBS})
target_include_directories(linq-static PUBLIC ${INCS})
target_include_directories(linq-static PUBLIC ./src)
set_target_properties(linq-static PROPERTIES
  PUBLIC_HEADER src/linq.h
  OUTPUT_NAME linq)
target_compile_definitions(linq-static PUBLIC
  "LINQ_STATIC"
  "${DEFS}")
install(TARGETS linq-static
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib
        RUNTIME DESTINATION bin
        PUBLIC_HEADER DESTINATION include/altronix)
