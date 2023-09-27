import subprocess
import os
import platform
import sys
import json

script_dir = os.path.dirname(os.path.abspath(__file__))
options_cache_dir = os.path.join(script_dir, "options_cache.json")
compiler = "Visual Studio 17 2022"
toolchain_path = ""
source_dir_arg = ".."
build_dir_arg = "build"
build_type_arg = "Debug"
binary_dir_arg = build_type_arg + "-Build"
install_dir_arg = "Install"
source_dir = ""
build_dir = ""
binary_dir = ""
install_dir = ""
os_name = ""
platform_name = ""
release_name = ""

# CACHE OPERATIONS


def check_cache():
    if not os.path.exists(options_cache_dir):
        with open(options_cache_dir, "w") as cache_file:
            json.dump({}, cache_file)
        print("Cache not found! Creating new cache file.")
        write_to_cache("first_setup", "False")
    else:
        print("Cache found!")


def write_to_cache(key, value):
    with open(options_cache_dir, "r") as cache_file:
        cache_data = json.load(cache_file)
    cache_data[key] = value
    with open(options_cache_dir, "w") as cache_file:
        json.dump(cache_data, cache_file)


def get_from_cache(key):
    try:
        with open(options_cache_dir, "r") as cache_file:
            cache_data = json.load(cache_file)
        return cache_data[key]
    except KeyError:
        return None


def print_cache():
    print("Cache contents:")
    with open(options_cache_dir, "r") as cache_file:
        cache_data = json.load(cache_file)
    for key, value in cache_data.items():
        print(key + ": " + value)


def edit_cache_entry():
    key = input("Enter key to edit (\'0\' to exit): ")
    if key == "0":
        return
    value = input("Enter new value: ")

    write_to_cache(key, value)

    print("Cache updated.")
    return


def edit_cache():
    print_cache()
    print("0. Exit")
    print("1. Edit cache entry")
    print("2. Delete and recreate cache file")
    choice = input("Enter: ")
    match choice:
        case "0":
            return
        case "1":
            edit_cache_entry()
        case "2":
            os.remove(options_cache_dir)
            check_cache()
            first_setup()

# END OF CACHE OPERATIONS

# SETTING UP GLOBAL VARIABLES


def first_setup():
    global platform_name
    check_platform()
    if (platform_name == "Windows"):
        find_vcpkg_root()
    write_to_cache("first_setup", "True")


def arg_first_setup(l_vcpkg_root):
    global platform_name
    check_platform()
    if (platform_name == "Windows"):
        write_to_cache("vcpkg_root", os.path.abspath(l_vcpkg_root))
    write_to_cache("first_setup", "True")


def check_platform():
    global os_name, platform_name, release_name
    os_name = get_from_cache("os_name")
    platform_name = get_from_cache("platform_name")
    release_name = get_from_cache("release_name")
    if os_name is None or platform_name is None or release_name is None:
        os_name = os.name
        platform_name = platform.system()
        release_name = platform.release()
        print(
            f"SYSTEM INFORMATION\nOS: {os_name}, PLATFORM: {platform_name}, RELEASE: {release_name}")
        write_to_cache("os_name", os_name)
        write_to_cache("platform_name", platform_name)
        write_to_cache("release_name", release_name)


def find_vcpkg_root():
    valid_path = False
    while not valid_path:
        vcpkg_root = input("Enter VCPKG root: ")
        normalized_vcpkg_root = os.path.normpath(vcpkg_root)
        if os.path.exists(os.path.join(normalized_vcpkg_root, "vcpkg.exe")):
            valid_path = True
        else:
            print("Invalid path. Try again.")
    write_to_cache("vcpkg_root", normalized_vcpkg_root)
    print("VCPKG root set to \"" + normalized_vcpkg_root + "\".")
    print("Additionally, you should set the following environment variables:\nCMAKE_TOOLCHAIN_FILE=" + normalized_vcpkg_root +
          "\\scripts\\buildsystems\\vcpkg.cmake" + "\nVCPKG_ROOT=" + normalized_vcpkg_root + "\nVCPKG_DEFAULT_TRIPLET=x64-windows")


def prepare_cmake_variables():
    check_platform()
    global source_dir, build_dir, binary_dir, toolchain_path, install_dir
    source_dir = os.path.join(script_dir, source_dir_arg)
    build_dir = os.path.join(source_dir, build_dir_arg)
    binary_dir = os.path.join(source_dir, binary_dir_arg)
    install_dir = os.path.join(
        source_dir, install_dir_arg, platform_name, build_type_arg)
    if (platform_name == "Windows"):
        toolchain_path = os.path.abspath(os.path.join(get_from_cache(
            "vcpkg_root"), "scripts", "buildsystems", "vcpkg.cmake"))

# END OF SETTING UP GLOBAL VARIABLES

# PROJECT MANAGEMENT COMMANDS


def prepare_cmake_project():
    prepare_cmake_variables()

    # Create the build directory if it doesn't exist
    os.makedirs(build_dir, exist_ok=True)

    # Change directory to the build directory
    os.chdir(build_dir)

    # Run CMake to configure the project
    cmake_configure_cmd = [
        "cmake",
        "-S", source_dir,
        "-B", build_dir,
        "-G", compiler,
        "-DCMAKE_TOOLCHAIN_FILE=" + toolchain_path,
    ]
    print("Running: " + " ".join(cmake_configure_cmd))
    subprocess.run(cmake_configure_cmd)

# TODO: Should be expanded later.
# def prepare_cmake_project(l_compiler, l_toolchain, l_c_compiler, l_cxx_compiler):
#     global platform_name, source_dir, build_dir, compiler, toolchain_path
#     prepare_cmake_variables()

#     # Create the build directory if it doesn't exist
#     os.makedirs(build_dir, exist_ok=True)

#     # Change directory to the build directory
#     os.chdir(build_dir)

#     if (l_compiler is None and l_toolchain is None and l_c_compiler is None and l_cxx_compiler is None):
#         if (platform_name == "Windows"):
#             cmake_configure_cmd = [
#                 "cmake",
#                 "-S", source_dir,
#                 "-B", build_dir,
#                 "-G", "Visual Studio 17 2022",
#                 "-DCMAKE_TOOLCHAIN_FILE=" + toolchain_path,
#             ]

#     elif toolchain is None:
#         cmake_configure_cmd = [
#             "cmake",
#             "-S", source_dir,
#             "-B", build_dir,
#             "-G", l_compiler,
#             "-DCMAKE_C_COMPILER=" + l_c_compiler,
#             "-DCMAKE_CXX_COMPILER=" + l_cxx_compiler
#         ]
#     else:
#         # Run CMake to configure the project
#         cmake_configure_cmd = [
#             "cmake",
#             "-S", source_dir,
#             "-B", build_dir,
#             "-G", l_compiler,
#             "-DCMAKE_TOOLCHAIN_FILE=" + l_toolchain,
#             "-DCMAKE_C_COMPILER=" + l_c_compiler,
#             "-DCMAKE_CXX_COMPILER=" + l_cxx_compiler
#         ]

#     print("Running: " + " ".join(cmake_configure_cmd))
#     subprocess.run(cmake_configure_cmd)


def arg_prepare_cmake_project_build_type(l_build_type):
    match l_build_type:
        case "Debug":
            write_to_cache("build_type", "Debug")
        case "Release":
            write_to_cache("build_type", "Release")
        case _:
            raise Exception("Invalid build type!")


def arg_prepare_cmake_project(l_build_preset):
    global platform_name, source_dir, build_dir, compiler, toolchain_path
    prepare_cmake_variables()

    # Create the build directory if it doesn't exist
    os.makedirs(build_dir, exist_ok=True)

    # Change directory to the build directory
    os.chdir(build_dir)

    match l_build_preset:
        case "nt/msvc":
            cmake_configure_cmd = [
                "cmake",
                "-S", source_dir,
                "-B", build_dir,
                "-G", "Visual Studio 17 2022",
                "-DCMAKE_TOOLCHAIN_FILE=" + toolchain_path,
            ]
        case "unix/clang":
            cmake_configure_cmd = [
                "cmake",
                "-S", source_dir,
                "-B", build_dir,
                "-G", "Ninja",
                "-DCMAKE_C_COMPILER=" + "clang",
                "-DCMAKE_CXX_COMPILER=" + "clang++"
            ]
        case "unix/gcc":
            cmake_configure_cmd = [
                "cmake",
                "-S", source_dir,
                "-B", build_dir,
                "-G", "Ninja",
                "-DCMAKE_C_COMPILER=" + "gcc",
                "-DCMAKE_CXX_COMPILER=" + "g++"
            ]
        case _:
            print("Invalid build preset!")
            return

    print("Running: " + " ".join(cmake_configure_cmd))
    subprocess.run(cmake_configure_cmd)


def build_cmake_project():
    prepare_cmake_variables()
    # Run the build command (e.g., make, ninja, or other build system)
    build_cmd = [
        "cmake",
        "--build", build_dir,
        "--config", build_type_arg
    ]
    print("Running: " + " ".join(build_cmd))
    subprocess.run(build_cmd)


def arg_build_cmake_project(l_build_type):
    prepare_cmake_variables()
    # Run the build command (e.g., make, ninja, or other build system)
    build_cmd = [
        "cmake",
        "--build", build_dir,
        "--config", l_build_type
    ]
    print("Running: " + " ".join(build_cmd))
    subprocess.run(build_cmd)

# CURRENTLY DOES NOT WORK AS INTENDED WITH GTEST


def install_cmake_project():
    prepare_cmake_variables()
    # Run the install command to copy the built binaries to the custom location
    install_cmd = [
        "cmake",
        "--install", build_dir,
        "--prefix", os.path.join(source_dir, "Install",
                                 platform_name, build_type_arg),
        "--config", build_type_arg,
        "-v"
    ]
    print("Running: " + " ".join(install_cmd))
    subprocess.run(install_cmd)


def arg_install_cmake_project(l_build_type):
    prepare_cmake_variables()
    # Run the install command to copy the built binaries to the custom location
    install_cmd = [
        "cmake",
        "--install", build_dir,
        "--prefix", os.path.join(source_dir, "Install",
                                 platform_name, l_build_type),
        "--config", l_build_type,
        "-v"
    ]
    print("Running: " + " ".join(install_cmd))
    subprocess.run(install_cmd)


def clean_cmake_project():
    prepare_cmake_variables()

    print(
        f"WARNING\nThis will remove all CMAKE project files (generated configuration, build, and install).\nGIT CLEAN WILL BE USED ON {build_dir} AND {install_dir}\nMAKE SURE YOU HAVE NO UNCOMMITTED CHANGES IN THESE DIRECTORIES\n")
    print(f"CONTINUE? (Y/N)")
    choice = input("Enter: ")
    if choice != "Y" and choice != "y":
        print("Aborting.")
        return

    os.chdir(source_dir)

    # Run the clean command to remove all cmake project files (generated configuration, build, and install)
    git_clean_build = [
        "git",
        "clean",
        "-dfx",
        os.path.join(source_dir, build_dir_arg)
    ]
    print("Running: " + " ".join(git_clean_build))
    subprocess.run(git_clean_build)

    git_clean_install = [
        "git",
        "clean",
        "-dfx",
        os.path.join(source_dir, install_dir_arg)
    ]
    print("Running: " + " ".join(git_clean_install))
    subprocess.run(git_clean_install)

# END OF PROJECT MANAGEMENT COMMANDS

# MENU


def menu():
    while True:
        print("0. Exit")
        print("1. Prepare CMAKE project")
        print("2. Build CMAKE project")
        print("3. Install CMAKE project")
        print("4. Clean CMAKE project")
        print("5. Print cached variables")
        print("6. Edit cache")
        choice = input("Enter: ")
        match choice:
            case "0":
                return
            case "1":
                prepare_cmake_project()
            case "2":
                build_cmake_project()
            case "3":
                install_cmake_project()
            case "4":
                clean_cmake_project()
            case "5":
                print_cache()
            case "6":
                edit_cache()

# END OF MENU

# HELP INFO


def help():
    print('''usage:
          setup.py [NO ARGS] ---> run default menu
          setup.py [-h | --help] ---> print this help info
          setup.py [-cah | --cache-help] ---> print help info for cache
          setup.py [-ph | --project-help] ---> print help info for project management
          ''')


def cache_help():
    print('''usage:
          setup.py [-cg | --cache-generate <vcpkg_root>] ---> generate cache file
          setup.py [-ca | --cache] ---> print cache contents
          setup.py [-cae | --cache-edit] ---> edit cache
          setup.py [-cag | --cache-get <key>] ---> get value from cache
          ''')


def project_help():
    # setup.py [-pb | --project-build <build type>] ---> build CMake project (Debug/Release)

    print('''usage:
          setup.py [-pg | --project-generate <system-type> <build-type>] ---> generate CMake project
          System types:
            nt/msvc ---> Windows, MSVC compiler
            unix/clang ---> Unix, Clang compiler
            unix/gcc ---> Unix, GCC compiler
          Build types:
            Debug ---> Debug build
            Release ---> Release build
          setup.py [-pb | --project-build <build preset>] ---> build CMake project (nt/msvc, unix/clang, unix/gcc)
          setup.py [-pi | --project-install <build type>] ---> install CMake project (Debug/Release)
          ''')

# END OF HELP INFO


if __name__ == "__main__":
    if (len(sys.argv) == 1):
        check_cache()
        try:
            if (get_from_cache("first_setup") == "False"):
                first_setup()
        except None:
            print("Failed parsing. Delete cache file and try again.")
        menu()
        exit()
    if (len(sys.argv) >= 2):
        match sys.argv[1]:
            case "-h" | "--help":
                help()
            # Cache operations
            case "-cah" | "--cache-help":
                cache_help()
            case "-cg" | "--cache-generate":
                check_cache()
                if (len(sys.argv) == 3):
                    arg_first_setup(sys.argv[2])
                else:
                    first_setup()
            case "-ca" | "--cache":
                print_cache()
            case "-cae" | "--cache-edit":
                edit_cache()
            case "-cag" | "--cache-get":
                if (sys.argv[2] is None):
                    print("No key specified!")
                    exit()
                print(get_from_cache(sys.argv[2]))
                exit()
            # Project operations
            case "-ph" | "--project-help":
                project_help()
            case "-pg" | "--project-generate":
                if (len(sys.argv) == 4):
                    arg_prepare_cmake_project_build_type(sys.argv[3])
                    arg_prepare_cmake_project(sys.argv[2])
                # if (len(sys.argv) == 2):
                #     prepare_cmake_project(None, None, None, None)
                # elif (len(sys.argv) == 3):
                #     prepare_cmake_project(sys.argv[2], None, None, None)
                # elif (len(sys.argv) == 4):
                #     prepare_cmake_project(sys.argv[2], sys.argv[3], None, None)
                # elif (len(sys.argv) == 5):
                #     prepare_cmake_project(
                #         sys.argv[2], sys.argv[3], sys.argv[4], None)
                # elif (len(sys.argv) == 6):
                #     prepare_cmake_project(
                #         sys.argv[2], sys.argv[3], sys.argv[4], sys.argv[5])
            case "-pb" | "--project-build":
                if (len(sys.argv) == 3):
                    arg_build_cmake_project(sys.argv[2])
            case "-pi" | "--project-install":
                if (len(sys.argv) == 3):
                    arg_install_cmake_project(sys.argv[2])
            case _:
                help()
