


cmake_minimum_required(VERSION |CMAKE_MINIMUM_VERSION|)
set(CMAKE_|CXX_OR_C_STANDARD|_STANDARD |CXX_OR_C_VERSION_NAME|)
set(|CXX_OR_C_STANDARD|_VERSION_NAME "|CXX_OR_C_STANDARD|_std_${CMAKE_|CXX_OR_C_STANDARD|_STANDARD}")
set(CMAKE_|CXX_OR_C_STANDARD|_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)

# TOGGLE TESTING.
set(MAIN_TEST |TESTING|)

# TOGGLE EXAMPLES.
set(EXAMPLES |EXAMPLES|)

# SETTING PROJECT VERSION.
set(PROJECT_VERSION_MAJOR 0)
set(PROJECT_VERSION_MINOR 0)
set(PROJECT_VERSION_PATCH 0)
set(PROJECT_VERSION_TWEAK 0)
set(PROJECT_VERSION_SUFFIX "a")
set(PROJECT_VERSION ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}.${PROJECT_VERSION_TWEAK})
set(PROJECT_FULL_VERSION ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}.${PROJECT_VERSION_TWEAK}.${PROJECT_VERSION_SUFFIX})

# CREATING PROJECT.
project(|PROJECT_NAME| VERSION ${PROJECT_VERSION})

# RUN SETUP.PY TO ASSIGN BUILD TYPE.
execute_process(
        COMMAND python "|BUILD_PY_LOCATION|" "-cag" "build_type"
        OUTPUT_VARIABLE BUILD_TYPE_SETUP_OUTPUT
        RESULT_VARIABLE BUILD_TYPE_SETUP_RESULT
        OUTPUT_STRIP_TRAILING_WHITESPACE
)
set(CMAKE_BUILD_TYPE ${BUILD_TYPE_SETUP_OUTPUT})

# SET INSTALLATION PATH BASED ON BUILD TYPE.
# SETTING BUILD TYPE.
if(CMAKE_BUILD_TYPE STREQUAL "Debug")
        set(INSTALL_PATH_SUFFIX "Debug")
        set(CMAKE_BUILD_TYPE Debug)

        # TOGGLE CUSTOM VERBOSITY.
        set(VERBOSE ON CACHE INTERNAL "Verbosity on")
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
        set(INSTALL_PATH_SUFFIX "Release")

        # TOGGLE CUSTOM VERBOSITY.
        set(VERBOSE OFF CACHE INTERNAL "Verbosity off")
else()
        set(INSTALL_PATH_SUFFIX "Unknown")

        # TOGGLE CUSTOM VERBOSITY.
        set(VERBOSE ON CACHE INTERNAL "Verbosity on")
endif()

# PATH TO INSTALLATION DIRECTORY.
set(CMAKE_INSTALL_PREFIX "${CMAKE_CURRENT_SOURCE_DIR}/Install/${CMAKE_BUILD_TYPE}" CACHE PATH "Installation Directory" FORCE)

if(${VERBOSE})
        message(STATUS "INSTALLATION SETUP PROCESS STARTING...")
        message(STATUS "DEFAULT INSTALLATION DIRECTORY: \"${CMAKE_INSTALL_PREFIX}\".")
endif()

# CHECKING PLATFORM.
set(CURRENT_PLATFORM "${CMAKE_SYSTEM_NAME}")

if(${VERBOSE})
        message(STATUS "CURRENT PLATFORM: ${CMAKE_SYSTEM_NAME}")
endif()

# DOUBLE CHECKS IF THE SYSTEM IS RUNNING ON WINDOWS-X64 SYSTEM.
# THIS ALSO FAILS IF THE COMPILER IS NOT 64-BIT.
if(NOT CMAKE_SIZEOF_VOID_P EQUAL 8)
        message(FATAL_ERROR "${CMAKE_PROJECT_NAME} SUPPORTS X64 SYSTEMS ONLY.")
        return()
endif()

# SETTING UP COMPILER.
if(${VERBOSE})
        message(STATUS "SETTING UP COMPILER...")
endif()

string(TOLOWER "${CMAKE_|CXX_OR_C_STANDARD|_COMPILER_ID}" LOWER_COMPILER_ID)

if(LOWER_COMPILER_ID STREQUAL "msvc")
        set(CURRENT_COMPILER "MSVC")
        set(COMPILATION_PLATFORM "WIN32")
elseif(LOWER_COMPILER_ID STREQUAL "gnu")
        set(CURRENT_COMPILER "GNU")
        set(COMPILATION_PLATFORM "UNIX")
elseif(LOWER_COMPILER_ID STREQUAL "clang")
        set(CURRENT_COMPILER "CLANG")
        set(COMPILATION_PLATFORM "UNIX")
else()
        message(FATAL_ERROR "COMPILER NOT SUPPORTED.")
endif()

if(${VERBOSE})
        message(STATUS "CURRENT COMPILER PATH: ${CMAKE_|CXX_OR_C_STANDARD|_COMPILER}")
        message(STATUS "CURRENT COMPILER: ${CMAKE_|CXX_OR_C_STANDARD|_COMPILER_ID}")
        message(STATUS "CURRENT COMPILER VERSION: ${CMAKE_|CXX_OR_C_STANDARD|_COMPILER_VERSION}")
        message(STATUS "COMPILER SETUP COMPLETE.")
endif()

# DEPENDENCY MANAGEMENT.
if(COMPILATION_PLATFORM STREQUAL "WIN32")
        set(USE_VCPKG |IF_VCPKG|)

        if(USE_VCPKG)

                if(${VERBOSE})
                        message(STATUS "VCPKG STATUS: ${USE_VCPKG}")
                endif()

                # RUN SETUP.PY TO GET VCPKG ROOT PATH.
                execute_process(
                        COMMAND python "$|BUILD_PY_LOCATION|" "-cag" "vcpkg_root"
                        OUTPUT_VARIABLE VCPKG_SETUP_OUTPUT
                        RESULT_VARIABLE VCPKG_SETUP_RESULT
                        OUTPUT_STRIP_TRAILING_WHITESPACE
                )

                set(VCPKG_FILEPATH "${VCPKG_SETUP_OUTPUT}")
                file(TO_CMAKE_PATH "${VCPKG_FILEPATH}/installed/x64-windows" VCPKG_CORE)

                if(NOT EXISTS "${VCPKG_FILEPATH}")
                        message(FATAL_ERROR "VCPKG NOT FOUND - MAKE SURE \"${VCPKG_FILEPATH}\" IS SET CORRECTLY.\nCURRENT PATH IS SET TO: \"${VCPKG_FILEPATH}\".")
                endif()

                if(USE_VCPKG)
                        if(${VERBOSE})
                                message(STATUS "LOOKING FOR VCPKG IN \"${VCPKG_FILEPATH}\" TO MANAGE DEPENDENCIES.")
                        endif()

                        # set(X_VCPKG_APPLOCAL_DEPS_INSTALL ON)
                        set(CMAKE_TOOLCHAIN_FILE "${VCPKG_FILEPATH}/scripts/buildsystems/vcpkg.cmake" CACHE STRING "Vcpkg toolchain file")
                        set(CMAKE_PREFIX_PATH "${VCPKG_FILEPATH}/installed/x64-windows")

                        if(${VERBOSE})
                                message(STATUS "VCPKG FOUND IN \"${VCPKG_FILEPATH}\".")
                        endif()
                endif()

        endif()
elseif(COMPILATION_PLATFORM STREQUAL "UNIX")
        set(USE_VCPKG OFF)

        if(${VERBOSE})
                message(STATUS "VCPKG STATUS: ${USE_VCPKG}")
        endif()

        if(USE_VCPKG)
                message(FATAL_ERROR "VCPKG NOT SUPPORTED ON UNIX SYSTEMS.")
        endif()
endif()

# ADDING DEPENDENCIES.
if(${VERBOSE})
        message(STATUS "LOOKING FOR DEPENDENCIES...")
endif()

|VCPKG_LIBS|

|IF_PYTHON|

|IF_BOOST|

# CHECKING PKGCONFIG.
if(PKG_CONFIG_FOUND)
        message(STATUS "PKGCONFIG STATUS: ${PKG_CONFIG_FOUND}")

        |PKG_CONFIG_LIBS|

else()
        message(FATAL_ERROR "pkg-config not found")
endif()

# FINAL CHECK ON DEPENDENCIES.
|IF_PYTHON|

|IF_STB|

if(${VERBOSE})
        message(STATUS "ALL DEPENDENCIES FOUND.")
endif()

# ADDING TESTING ENVIRONMENT.
if(${VERBOSE})
        message(STATUS "ADDING TESTING ENVIRONMENT...")
endif()

include(CTest)
enable_testing()
set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_FULL_VERSION})
include(CPack)

if(${VERBOSE})
        message(STATUS "TESTING ENVIRONMENT ADDED.")
endif()

# BOOST SETTINGS
|IF_BOOST|

# ADDITIONAL INFORMATION.
if(${VERBOSE})
        message(STATUS "CURRENT BUILD TYPE: ${CMAKE_BUILD_TYPE}")
        message(STATUS "PROJECT VERSION: \"${PROJECT_FULL_VERSION}\".")
        message(STATUS "TOOLCHAIN STATUS: \"${CMAKE_TOOLCHAIN_FILE}\".")
        message(STATUS "CMAKE INCLUDE CURRENT DIRECTORY: \"${CMAKE_INCLUDE_CURRENT_DIR}\".")
        message(STATUS "CTEST STATUS: \"${CPACK_PROJECT_NAME}\" VERSION \"${CPACK_PROJECT_VERSION}\".")
endif()

# CHECK IF SYSTEMS IS RUNNING WINDOWS AND ADD NECESSARY ATTRIBUTES.
# EVEN THOUGH CURRENTLY ONLY WINDOWS IS SUPPORTED, ADDING THIS CHECK WILL MAKE IT EASIER TO ADD SUPPORT FOR OTHER SYSTEMS IN THE FUTURE.
if(COMPILATION_PLATFORM STREQUAL "WIN32")
        # INSTALLATION FOR LIBRARIES.

        # ADD SOURCE FILES
        # |EXE_NAME| .CPP FILES
        file(GLOB_RECURSE |EXE_NAME|_CPP
                #"${PROJECT_SOURCE_DIR}/src/*.cpp"
        )

        # |EXE_NAME| .H FILES
        file(GLOB_RECURSE |EXE_NAME|_H
                #"${PROJECT_SOURCE_DIR}/src/*.h"
        )

        if(${VERBOSE})
                message(STATUS "|EXE_NAME| SOURCE FILES ADDED.")
        endif()

        |IF_LIBRARY|
        # add_library(|LIB_NAME| STATIC ${|LIB_NAME|_CPP})

        add_executable(|EXE_NAME| ${|EXE_NAME|_CPP} ${|EXE_NAME|_H})
        target_compile_features(|EXE_NAME| PUBLIC ${|CXX_OR_C_STANDARD|_VERSION_NAME})
        set_target_properties(|EXE_NAME| PROPERTIES VERSION ${PROJECT_FULL_VERSION})

        set_target_properties(|EXE_NAME| PROPERTIES OUTPUT_NAME "|EXE_NAME|")
        target_link_libraries(|EXE_NAME| PUBLIC
                |VCPKG_LIBS_TARGETS|
        )

        #target_precompile_headers(|EXE_NAME| PUBLIC
        #        "${PROJECT_SOURCE_DIR}/src/PCH.h"
        #)

        target_include_directories(|EXE_NAME| PUBLIC
                ${|EXE_NAME|_H}
                "${PROJECT_SOURCE_DIR}/src"
                ${OPENGL_INCLUDE_DIR}
                ${raylib_INCLUDE_DIRS}
                ${Python3_INCLUDE_DIRS}
                ${Boost_INCLUDE_DIRS}
                ${GTKMM_LIB_INCLUDE_DIRS}
        )

        # COMPILER DEFINITIONS FOR |EXE_NAME|.
        target_compile_definitions(|EXE_NAME| PUBLIC |PROJECT_NAME|_PLATFORM_WINDOWS)
        target_compile_definitions(|EXE_NAME| PUBLIC |PROJECT_NAME|_STATIC_LIBRARY)
        target_compile_definitions(|EXE_NAME| PUBLIC |PROJECT_NAME|_ENABLE_ASSERTS)
        target_compile_definitions(|EXE_NAME| PUBLIC |PROJECT_NAME|_COMPILER_${CURRENT_COMPILER})

        # ENABLE PROFILING FOR DEBUG BUILDS.
        if(CMAKE_BUILD_TYPE STREQUAL Debug)
                target_compile_definitions(|EXE_NAME| PUBLIC |PROJECT_NAME|_DEBUG_PROFILING)
                target_compile_definitions(|EXE_NAME| PUBLIC |PROJECT_NAME|_ENABLE_DEBUG_LOG)
        endif(CMAKE_BUILD_TYPE STREQUAL Debug)

        if(${VERBOSE})
                get_target_property(DEFINITIONS_LIST |EXE_NAME| COMPILE_DEFINITIONS)
                message(STATUS "|EXE_NAME| COMPILE DEFINITIONS: ${DEFINITIONS_LIST}")
        endif()

        if(${MAIN_TEST})
                add_subdirectory("${PROJECT_SOURCE_DIR}/tests")
        endif()

        if(${EXAMPLES})
                add_subdirectory("${PROJECT_SOURCE_DIR}/examples")
        endif()

        # INSTALLATION PROCEDURE.
        install(TARGETS |EXE_NAME|
                LIBRARY DESTINATION bin
                ARCHIVE DESTINATION bin
                RUNTIME DESTINATION bin)

        # ADD .DLLS TO INSTALL IF NEEDED

endif()

message(STATUS "|EXE_NAME|/CMAKE SUCCESSFULLY FINISHED.")
