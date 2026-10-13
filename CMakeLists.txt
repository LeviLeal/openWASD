cmake_minimum_required(VERSION 3.16)
project(wasd_anywhere VERSION 0.1.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

find_package(Qt6 COMPONENTS Widgets REQUIRED)
find_package(PkgConfig REQUIRED)
pkg_check_modules(LIBEVDEV REQUIRED libevdev)

include_directories(include ${LIBEVDEV_INCLUDE_DIRS})

set(SOURCES
    src/main.cpp
    src/mainwindow.cpp
    src/mapping_manager.cpp
    src/key_binding_button.cpp
    src/virtual_gamepad.cpp
    src/keyboard_interceptor.cpp
)

set(HEADERS
    include/mainwindow.h
    include/mapping_manager.h
    include/key_binding_button.h
    include/gamepad_types.h
    include/virtual_gamepad.h
    include/keyboard_interceptor.h
    include/key_codes.h
)

add_executable(wasd-gui ${SOURCES} ${HEADERS} resources.qrc)
target_link_libraries(wasd-gui PRIVATE Qt6::Widgets ${LIBEVDEV_LIBRARIES})
