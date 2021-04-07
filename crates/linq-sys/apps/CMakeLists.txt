add_executable(scan scan.c)
target_link_libraries(scan linq-static)
install(TARGETS scan
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib
        RUNTIME DESTINATION bin
        PUBLIC_HEADER DESTINATION include/altronix)
