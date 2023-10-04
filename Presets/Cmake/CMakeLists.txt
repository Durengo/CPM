|CMAKE_MINIMUM_VERSION|
|CXX_OR_C_STANDARD|
|CXX_OR_C_VERSION_NAME|
cmake_minimum_required(VERSION 3.22)
set(CMAKE_CXX_STANDARD 23)
set(CXX_VERSION_NAME "cxx_std_${CMAKE_CXX_STANDARD}")
set(CMAKE_CXX_STANDARD_REQUIRED ON)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)

# TOGGLE TESTING.
set(MAIN_TEST ON)

# TOGGLE EXAMPLES.
set(EXAMPLES ON)

# SETTING PROJECT VERSION.
set(PROJECT_VERSION_MAJOR 0)
set(PROJECT_VERSION_MINOR 0)
set(PROJECT_VERSION_PATCH 0)
set(PROJECT_VERSION_TWEAK 0)
set(PROJECT_VERSION_SUFFIX "a")
set(PROJECT_VERSION ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}.${PROJECT_VERSION_TWEAK})
set(PROJECT_FULL_VERSION ${PROJECT_VERSION_MAJOR}.${PROJECT_VERSION_MINOR}.${PROJECT_VERSION_PATCH}.${PROJECT_VERSION_TWEAK}.${PROJECT_VERSION_SUFFIX})

# CREATING PROJECT.
project(PROJECT_NAME! VERSION ${PROJECT_VERSION})

# RUN SETUP.PY TO ASSIGN BUILD TYPE.
execute_process(
        COMMAND python "${PROJECT_SOURCE_DIR}/utils/build.py" "-cag" "build_type"
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
        message(FATAL_ERROR "OMNI-GRID-FRAMEWORK SUPPORTS X64 SYSTEMS ONLY.")
        return()
endif()

# SETTING UP COMPILER.
if(${VERBOSE})
        message(STATUS "SETTING UP COMPILER...")
endif()

string(TOLOWER "${CMAKE_CXX_COMPILER_ID}" LOWER_COMPILER_ID)

if(LOWER_COMPILER_ID STREQUAL "msvc")
        set(CURRENT_COMPILER "MSVC")
        set(COMPILATION_PLATFORM "WIN32")

# elseif(LOWER_COMPILER_ID STREQUAL "gnu")
# set(CURRENT_COMPILER "GNU")
# set(COMPILATION_PLATFORM "UNIX")
# elseif(LOWER_COMPILER_ID STREQUAL "clang")
# set(CURRENT_COMPILER "CLANG")
# set(COMPILATION_PLATFORM "UNIX")
else()
        message(FATAL_ERROR "COMPILER NOT SUPPORTED.")
endif()

if(${VERBOSE})
        message(STATUS "CURRENT COMPILER PATH: ${CMAKE_CXX_COMPILER}")
        message(STATUS "CURRENT COMPILER: ${CMAKE_CXX_COMPILER_ID}")
        message(STATUS "CURRENT COMPILER VERSION: ${CMAKE_CXX_COMPILER_VERSION}")
        message(STATUS "COMPILER SETUP COMPLETE.")
endif()

# DEPENDENCY MANAGEMENT.
if(COMPILATION_PLATFORM STREQUAL "WIN32")
        set(USE_VCPKG ON)

        if(${VERBOSE})
                message(STATUS "VCPKG STATUS: ${USE_VCPKG}")
        endif()

        # RUN SETUP.PY TO GET VCPKG ROOT PATH.
        execute_process(
                COMMAND python "${PROJECT_SOURCE_DIR}/utils/build.py" "-cag" "vcpkg_root"
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
elseif(COMPILATION_PLATFORM STREQUAL "UNIX")
        set(USE_VCPKG OFF)

        if(${VERBOSE})
                message(STATUS "VCPKG STATUS: ${USE_VCPKG}")
        endif()

        meesage(FATAL_ERROR "VCPKG NOT SUPPORTED ON UNIX SYSTEMS.")
endif()

# ADDING DEPENDENCIES.
if(${VERBOSE})
        message(STATUS "LOOKING FOR DEPENDENCIES...")
endif()

find_package(Git QUIET)
find_package(fmt CONFIG REQUIRED)
find_package(spdlog CONFIG REQUIRED)
find_package(OpenGL REQUIRED)
find_package(raylib CONFIG REQUIRED)
find_package(PkgConfig REQUIRED)

# SETTING UP PYTHON.
find_package(Python3 REQUIRED COMPONENTS Development Interpreter)

if(${VERBOSE})
        message(STATUS "SETTING PYTHON VERSION...")
endif()

set(PYTHON_VERSION "python${Python3_VERSION_MAJOR}${Python3_VERSION_MINOR}")

if(${VERBOSE})
        message(STATUS "PYTHON VERSION: ${PYTHON_VERSION}")
endif()

find_package(Boost REQUIRED COMPONENTS ${PYTHON_VERSION} system)

# CHECKING PKGCONFIG.
if(PKG_CONFIG_FOUND)
        message(STATUS "PKGCONFIG STATUS: ${PKG_CONFIG_FOUND}")

        # CHECKING GTKMM.
        pkg_check_modules(GTKMM_LIB REQUIRED IMPORTED_TARGET gtkmm-4.0)

        if(GTKMM_LIB_FOUND)
                message(STATUS "GTKmm found")

        else()
                message(FATAL_ERROR "GTKmm not found")
        endif()

else()
        message(FATAL_ERROR "pkg-config not found")
endif()

# FINAL CHECK ON DEPENDENCIES.
if(${VERBOSE})
        message(STATUS "PYTHON LIBRARIES: ${Python3_LIBRARIES}")
        message(STATUS "PYTHON EXECUTABLE: ${Python3_EXECUTABLE}")
        message(STATUS "PYTHON INCLUDE DIRS: ${Python3_INCLUDE_DIRS}")
        message(STATUS "BOOST LIBRARIES: ${Boost_LIBRARIES}")
endif()

if(CURRENT_PLATFORM STREQUAL "WIN32")
        find_package(Stb REQUIRED)
endif()

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
if(${VERBOSE})
        message(STATUS "SETTING BOOST LIBRARY OPTIONS...")
endif()

set(Boost_USE_STATIC_LIBS ON)
set(Boost_USE_STATIC_RUNTIME ON)
set(Boost_USE_MULTITHREADED ON)

if(CMAKE_BUILD_TYPE STREQUAL "Debug")
        set(Boost_USE_RELEASE_LIBS OFF)
        set(Boost_USE_DEBUG_LIBS ON)
elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
        set(Boost_USE_RELEASE_LIBS ON)
        set(Boost_USE_DEBUG_LIBS OFF)
else()
        set(Boost_USE_RELEASE_LIBS OFF)
        set(Boost_USE_DEBUG_LIBS ON)
endif()

if(${VERBOSE})
        message(STATUS "BOOST LIBRARY OPTIONS SET.")
endif()

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
        # GTK DEBUG DLLS.
        set(GTK_DLL_DEBUG_FILES
                ${VCPKG_CORE}/debug/bin/brotlicommon.dll
                ${VCPKG_CORE}/debug/bin/brotlidec.dll
                ${VCPKG_CORE}/debug/bin/bz2d.dll
                ${VCPKG_CORE}/debug/bin/cairo-2.dll
                ${VCPKG_CORE}/debug/bin/cairo-gobject-2.dll
                ${VCPKG_CORE}/debug/bin/cairomm-1.16-1.dll
                ${VCPKG_CORE}/debug/bin/cairo-script-interpreter-2.dll
                ${VCPKG_CORE}/debug/bin/epoxy-0.dll
                ${VCPKG_CORE}/debug/bin/fontconfig-1.dll
                ${VCPKG_CORE}/debug/bin/freetyped.dll
                ${VCPKG_CORE}/debug/bin/fribidi-0.dll
                ${VCPKG_CORE}/debug/bin/gdk_pixbuf-2.0-0.dll
                ${VCPKG_CORE}/debug/bin/gio-2.0-0.dll
                ${VCPKG_CORE}/debug/bin/giomm-2.68-1.dll
                ${VCPKG_CORE}/debug/bin/glib-2.0-0.dll
                ${VCPKG_CORE}/debug/bin/glibmm-2.68-1.dll
                ${VCPKG_CORE}/debug/bin/gmodule-2.0-0.dll
                ${VCPKG_CORE}/debug/bin/gobject-2.0-0.dll
                ${VCPKG_CORE}/debug/bin/graphene-1.0-0.dll
                ${VCPKG_CORE}/debug/bin/gtk-4-1.dll
                ${VCPKG_CORE}/debug/bin/gtkmm-4.0-0.dll
                ${VCPKG_CORE}/debug/bin/harfbuzz.dll
                ${VCPKG_CORE}/debug/bin/iconv-2.dll
                ${VCPKG_CORE}/debug/bin/intl-8.dll
                ${VCPKG_CORE}/debug/bin/jpeg62.dll
                ${VCPKG_CORE}/debug/bin/libexpatd.dll
                ${VCPKG_CORE}/debug/bin/liblzma.dll
                ${VCPKG_CORE}/debug/bin/libpng16d.dll
                ${VCPKG_CORE}/debug/bin/lzo2.dll
                ${VCPKG_CORE}/debug/bin/pango-1.0-0.dll
                ${VCPKG_CORE}/debug/bin/pangocairo-1.0-0.dll
                ${VCPKG_CORE}/debug/bin/pangoft2-1.0-0.dll
                ${VCPKG_CORE}/debug/bin/pangomm-2.48-1.dll
                ${VCPKG_CORE}/debug/bin/pangowin32-1.0-0.dll
                ${VCPKG_CORE}/debug/bin/pcre2-8d.dll
                ${VCPKG_CORE}/debug/bin/pixman-1-0.dll
                ${VCPKG_CORE}/debug/bin/sigc-3.0.dll
                ${VCPKG_CORE}/debug/bin/tiffd.dll
                ${VCPKG_CORE}/debug/bin/zlibd1.dll
        )

        if(EXISTS "${VCPKG_CORE}/debug/bin/ffi-8.dll")
                list(APPEND GTK_DLL_DEBUG_FILES "${VCPKG_CORE}/debug/bin/ffi-8.dll")
        elseif(EXISTS "${VCPKG_CORE}/debug/bin/libffi.dll")
                list(APPEND GTK_DLL_DEBUG_FILES "${VCPKG_CORE}/debug/bin/libffi.dll")
        else()
                message(WARNING "DEBUG FFI DLL NOT FOUND.")
        endif()

        # GTK RELEASE DLLS.
        set(GTK_DLL_RELEASE_FILES
                ${VCPKG_CORE}/bin/brotlicommon.dll
                ${VCPKG_CORE}/bin/brotlidec.dll
                ${VCPKG_CORE}/bin/bz2.dll
                ${VCPKG_CORE}/bin/cairo-2.dll
                ${VCPKG_CORE}/bin/cairo-gobject-2.dll
                ${VCPKG_CORE}/bin/cairomm-1.16-1.dll
                ${VCPKG_CORE}/bin/cairo-script-interpreter-2.dll
                ${VCPKG_CORE}/bin/epoxy-0.dll
                ${VCPKG_CORE}/bin/fontconfig-1.dll
                ${VCPKG_CORE}/bin/freetype.dll
                ${VCPKG_CORE}/bin/fribidi-0.dll
                ${VCPKG_CORE}/bin/gdk_pixbuf-2.0-0.dll
                ${VCPKG_CORE}/bin/gio-2.0-0.dll
                ${VCPKG_CORE}/bin/giomm-2.68-1.dll
                ${VCPKG_CORE}/bin/glib-2.0-0.dll
                ${VCPKG_CORE}/bin/glibmm-2.68-1.dll
                ${VCPKG_CORE}/bin/gmodule-2.0-0.dll
                ${VCPKG_CORE}/bin/gobject-2.0-0.dll
                ${VCPKG_CORE}/bin/graphene-1.0-0.dll
                ${VCPKG_CORE}/bin/gtk-4-1.dll
                ${VCPKG_CORE}/bin/gtkmm-4.0-0.dll
                ${VCPKG_CORE}/bin/harfbuzz.dll
                ${VCPKG_CORE}/bin/iconv-2.dll
                ${VCPKG_CORE}/bin/intl-8.dll
                ${VCPKG_CORE}/bin/jpeg62.dll
                ${VCPKG_CORE}/bin/libexpat.dll
                ${VCPKG_CORE}/bin/liblzma.dll
                ${VCPKG_CORE}/bin/libpng16.dll
                ${VCPKG_CORE}/bin/lzo2.dll
                ${VCPKG_CORE}/bin/pango-1.0-0.dll
                ${VCPKG_CORE}/bin/pangocairo-1.0-0.dll
                ${VCPKG_CORE}/bin/pangoft2-1.0-0.dll
                ${VCPKG_CORE}/bin/pangomm-2.48-1.dll
                ${VCPKG_CORE}/bin/pangowin32-1.0-0.dll
                ${VCPKG_CORE}/bin/pcre2-8.dll
                ${VCPKG_CORE}/bin/pixman-1-0.dll
                ${VCPKG_CORE}/bin/sigc-3.0.dll
                ${VCPKG_CORE}/bin/tiff.dll
                ${VCPKG_CORE}/bin/zlib1.dll
        )

        if(EXISTS "${VCPKG_CORE}/bin/ffi-8.dll")
                list(APPEND GTK_DLL_RELEASE_FILES "${VCPKG_CORE}/bin/ffi-8.dll")
        elseif(EXISTS "${VCPKG_CORE}/bin/libffi.dll")
                list(APPEND GTK_DLL_RELEASE_FILES "${VCPKG_CORE}/bin/libffi.dll")
        else()
                message(WARNING "DEBUG FFI DLL NOT FOUND.")
        endif()

        # BOOST PYTHON DEBUG DLLS.
        set(BOOST_PYTHON_DLL_DEBUG_FILES
                ${VCPKG_CORE}/debug/bin/boost_${PYTHON_VERSION}-vc143-mt-gyd-x64-1_82.dll
                ${VCPKG_CORE}/bin/boost_${PYTHON_VERSION}-vc143-mt-x64-1_82.dll
                ${VCPKG_CORE}/debug/bin/${PYTHON_VERSION}_d.dll
                ${VCPKG_CORE}/bin/${PYTHON_VERSION}.dll
                ${VCPKG_CORE}/debug/bin/zlibd1.dll
                ${VCPKG_CORE}/bin/zlib1.dll
        )

        # BOOST PYTHON RELEASE DLLS.
        set(BOOST_PYTHON_DLL_RELEASE_FILES
                ${VCPKG_CORE}/bin/boost_${PYTHON_VERSION}-vc143-mt-x64-1_82.dll
                ${VCPKG_CORE}/bin/${PYTHON_VERSION}.dll
                ${VCPKG_CORE}/bin/zlib1.dll
        )

        # RAYLIB DEBUG DLLS.
        set(RAYLIB_DLL_DEBUG_FILES
                ${VCPKG_CORE}/debug/bin/raylib.dll
        )

        # RAYLIB RELEASE DLLS.
        set(RAYLIB_DLL_RELEASE_FILES
                ${VCPKG_CORE}/bin/raylib.dll
        )

        # ADD SOURCE FILES
        # OMNI-GRID .CPP FILES
        file(GLOB_RECURSE OMNI-GRID_CPP
                "${PROJECT_SOURCE_DIR}/omnigrid/ogrid/Log/*.cpp"
        )

        # OMNI-GRID .H FILES
        file(GLOB_RECURSE OMNI-GRID_H
                "${PROJECT_SOURCE_DIR}/omnigrid/ogrid/ogrid.h"
                "${PROJECT_SOURCE_DIR}/omnigrid/ogrid/Core/*.h"
                "${PROJECT_SOURCE_DIR}/omnigrid/ogrid/Log/*.h"
        )

        if(${VERBOSE})
                message(STATUS "OMNI-GRID SOURCE FILES ADDED.")
        endif()

        add_library(OMNI-GRID STATIC ${OMNI-GRID_CPP})

        # add_executable(OMNI-GRID ${OMNI-GRID_CPP} ${OMNI-GRID_H})
        target_compile_features(OMNI-GRID PUBLIC ${CXX_VERSION_NAME})
        set_target_properties(OMNI-GRID PROPERTIES VERSION ${PROJECT_FULL_VERSION})

        set_target_properties(OMNI-GRID PROPERTIES OUTPUT_NAME "OMNI-GRID")
        target_link_libraries(OMNI-GRID PUBLIC
                fmt::fmt
                spdlog::spdlog
                OpenGL::GL
                ${OPENGL_LIBRARIES}
                ${raylib_LIBRARIES}
                ${Python3_LIBRARIES}
                ${Boost_LIBRARIES}
                ${VCPKG_CORE}/lib/${PYTHON_VERSION}.lib
                PkgConfig::GTKMM_LIB
        )

        target_precompile_headers(OMNI-GRID PUBLIC
                "${PROJECT_SOURCE_DIR}/omnigrid/ogrid/PCH.h"
        )

        target_include_directories(OMNI-GRID PUBLIC
                ${OMNI-GRID_H}
                "${PROJECT_SOURCE_DIR}/omnigrid/ogrid"
                ${OPENGL_INCLUDE_DIR}
                ${raylib_INCLUDE_DIRS}
                ${Python3_INCLUDE_DIRS}
                ${Boost_INCLUDE_DIRS}
                ${GTKMM_LIB_INCLUDE_DIRS}
        )

        # COMPILER DEFINITIONS FOR OMNI-GRID.
        target_compile_definitions(OMNI-GRID PUBLIC OGRID_PLATFORM_WINDOWS)
        target_compile_definitions(OMNI-GRID PUBLIC OGRID_STATIC_LIBRARY)
        target_compile_definitions(OMNI-GRID PUBLIC OGRID_ENABLE_ASSERTS)
        target_compile_definitions(OMNI-GRID PUBLIC OGRID_COMPILER_${CURRENT_COMPILER})

        # ENABLE PROFILING FOR DEBUG BUILDS.
        if(CMAKE_BUILD_TYPE STREQUAL Debug)
                target_compile_definitions(OMNI-GRID PUBLIC OGRID_DEBUG_PROFILING)
                target_compile_definitions(OMNI-GRID PUBLIC OGRID_ENABLE_DEBUG_LOG)
        endif(CMAKE_BUILD_TYPE STREQUAL Debug)

        if(${VERBOSE})
                get_target_property(DEFINITIONS_LIST OMNI-GRID COMPILE_DEFINITIONS)
                message(STATUS "OMNI-GRID COMPILE DEFINITIONS: ${DEFINITIONS_LIST}")
        endif()

        if(${MAIN_TEST})
                add_subdirectory("${PROJECT_SOURCE_DIR}/tests")

                # add_executable(OMNI-GRID_TEST_ENV "${PROJECT_SOURCE_DIR}/tests/main.cpp")
                # target_link_libraries(OMNI-GRID_TEST_ENV PUBLIC OMNI-GRID)

                # message("OMNI-GRID MAIN CPP TEST EXECUTABLE ADDED.")
        endif()

        if(${EXAMPLES})
                add_subdirectory("${PROJECT_SOURCE_DIR}/examples")
        endif()

        # INSTALLATION PROCEDURE.
        install(TARGETS OMNI-GRID
                LIBRARY DESTINATION bin
                ARCHIVE DESTINATION bin
                RUNTIME DESTINATION bin)

        if(CMAKE_BUILD_TYPE STREQUAL "Debug")
                install(FILES ${GTK_DLL_DEBUG_FILES} DESTINATION bin)

                # install(FILES ${RAYLIB_DLL_DEBUG_FILES} DESTINATION bin)
                install(FILES ${BOOST_PYTHON_DLL_DEBUG_FILES} DESTINATION bin)
        elseif(CMAKE_BUILD_TYPE STREQUAL "Release")
                install(FILES ${GTK_DLL_RELEASE_FILES} DESTINATION bin)

                # install(FILES ${RAYLIB_DLL_RELEASE_FILES} DESTINATION bin)
                install(FILES ${BOOST_PYTHON_DLL_RELEASE_FILES} DESTINATION bin)
        endif()

        # TODO: Find a way to fix this issue without having to run a script or manually copying the dlls.
        # install(SCRIPT "${CMAKE_CURRENT_SOURCE_DIR}/utils/core_copy_missing_dlls.cmake")
endif()

message(STATUS "OMNI-GRID/CMAKE SUCCESSFULLY FINISHED.")
