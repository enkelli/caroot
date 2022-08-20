set(CAROOT_SOURCES
	ca.cpp
	main.cpp
	rules.cpp
)

add_executable(caroot ${CAROOT_SOURCES})
target_include_directories(caroot PUBLIC "${SRC_DIR}")
target_link_libraries(caroot PUBLIC
	inireader
)
install(TARGETS caroot DESTINATION "${INSTALL_BIN_DIR}")
