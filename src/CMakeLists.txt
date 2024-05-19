# SPDX-License-Identifier: BSD-2-Clause
# SPDX-FileCopyrightText: %{CURRENT_YEAR} %{AUTHOR} <%{EMAIL}>

# Target: static library
qt_add_qml_module(jewels-desktop_static
    STATIC
    URI org.kde.jewels-desktop
    VERSION 1.0
    QML_FILES
        contents/ui/Main.qml
        contents/ui/About.qml
)

target_sources(jewels-desktop_static PUBLIC
    app.cpp
)

target_link_libraries(jewels-desktop_static PUBLIC
    Qt6::Core
    Qt6::Gui
    Qt6::Qml
    Qt6::Quick
    Qt6::QuickControls2
    Qt6::Svg
    KF6::I18n
    KF6::CoreAddons
    KF6::ConfigCore
    KF6::ConfigGui
)
target_include_directories(jewels-desktop_static PUBLIC ${CMAKE_BINARY_DIR})

if (ANDROID)
    kirigami_package_breeze_icons(ICONS
        list-add
        help-about
        application-exit
        applications-graphics
    )
else()
    target_link_libraries(jewels-desktop_static PUBLIC Qt::Widgets)
endif()

kconfig_add_kcfg_files(jewels-desktop_static GENERATE_MOC jewels-desktopconfig.kcfgc)

# Target: main executable
add_executable(jewels-desktop main.cpp)
target_link_libraries(jewels-desktop PUBLIC jewels-desktop_static jewels-desktop_staticplugin)
install(TARGETS jewels-desktop ${KDE_INSTALL_TARGETS_DEFAULT_ARGS})
