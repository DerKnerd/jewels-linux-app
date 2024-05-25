set(CMAKE_AUTORCC ON)

# Target: static library
qt_add_qml_module(jewels_static
        STATIC
        URI dev.imanuel.jewels
        VERSION 1.0
        QML_FILES
        contents/ui/main.qml
)
#
#target_sources(jewels_static PRIVATE
#        app.cpp
#        Jewels.cpp
#)

target_link_libraries(jewels_static PUBLIC
        Qt6::Core
        Qt6::Concurrent
        Qt6::DBus
        Qt6::Gui
        Qt6::Qml
        Qt6::Quick
        Qt6::QuickControls2
        Qt6::Svg
        KF6::I18n
        KF6::CoreAddons
        KF6::ConfigCore
        KF6::ConfigGui
        KF6::IconThemes
)

target_include_directories(jewels_static PRIVATE ${CMAKE_BINARY_DIR})

target_link_libraries(jewels_static PUBLIC Qt::Widgets)

kconfig_add_kcfg_files(jewels_static GENERATE_MOC jewelsconfig.kcfgc)

set(icons_src)

ecm_add_app_icon(icons_src ICONS
        sc-apps-jewels.svg
)

# Target: main executable
add_executable(jewels
        ${icons_src}
        resources.qrc
        main.cpp
        app.cpp
        Jewels.cpp
        device/Bios.cpp
        device/Mainboard.cpp
        device/Kernel.cpp
        device/OperatingSystem.cpp
        device/Utils.cpp
        device/Device.cpp
        device/Drive.cpp
        device/Cpu.cpp
)

target_link_libraries(jewels PRIVATE jewels_static jewels_staticplugin)

install(TARGETS jewels ${KDE_INSTALL_TARGETS_DEFAULT_ARGS})
