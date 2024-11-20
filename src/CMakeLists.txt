set(CMAKE_AUTORCC ON)

find_package(CxxQt QUIET)

if(NOT CxxQt_FOUND)
    include(FetchContent)
    FetchContent_Declare(
            CxxQt
            GIT_REPOSITORY https://github.com/kdab/cxx-qt-cmake.git
            GIT_TAG v0.7.0
    )

    FetchContent_MakeAvailable(CxxQt)
endif()

cxx_qt_import_crate(
        MANIFEST_PATH detector-rs/Cargo.toml
        CRATES detector_rs
        QT_MODULES Qt::Core Qt::Gui Qt::Qml Qt::QuickControls2)

cxx_qt_import_qml_module(jewels_detector_rs
        URI "cloud.ulbricht.jewels"
        SOURCE_CRATE detector_rs)

add_library(jewels_static)

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
        qml/resources.qrc
        jewels/main.cpp
        jewels/app.cpp
)

target_link_libraries(jewels PRIVATE jewels_static jewels_detector_rs)

qt_import_qml_plugins(jewels)

install(TARGETS jewels ${KDE_INSTALL_TARGETS_DEFAULT_ARGS})
