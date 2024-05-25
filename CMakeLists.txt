cmake_minimum_required(VERSION 3.16)

project(jewels VERSION 1.0)

include(FeatureSummary)

set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOMOC ON)

set(NO_OCL ON)
set(QT6_MIN_VERSION 6.5.0)
set(KF6_MIN_VERSION 6.0.0)
set(CMAKE_CXX_STANDARD 23)

find_package(ECM ${KF6_MIN_VERSION} REQUIRED NO_MODULE)

set(CMAKE_MODULE_PATH ${CMAKE_MODULE_PATH} ${ECM_MODULE_PATH})

include(FeatureSummary)
include(KDEInstallDirs)
include(KDECMakeSettings)
include(ECMSetupVersion)
include(ECMFindQmlModule)
include(ECMInstallIcons)
include(ECMAddAppIcon)
include(KDEGitCommitHooks)

ecm_setup_version(${PROJECT_VERSION}
        VARIABLE_PREFIX JEWELS
        VERSION_HEADER "${CMAKE_CURRENT_BINARY_DIR}/src/version-jewels.h"
)

set(QT_EXTRA_COMPONENTS)
list(APPEND QT_EXTRA_COMPONENTS Widgets)

find_package(Qt6 ${QT6_MIN_VERSION} REQUIRED COMPONENTS Core Concurrent DBus Gui Qml QuickControls2 Svg ${QT_EXTRA_COMPONENTS})
find_package(KF6 ${KF6_MIN_VERSION} REQUIRED COMPONENTS Kirigami CoreAddons Config I18n IconThemes)

qt_policy(SET QTP0001 NEW)

ecm_find_qmlmodule(org.kde.kirigamiaddons.formcard 1.0)

add_subdirectory(src)

install(FILES dev.imanuel.jewels.desktop DESTINATION ${KDE_INSTALL_APPDIR})
install(FILES dev.imanuel.jewels.metainfo.xml DESTINATION ${KDE_INSTALL_METAINFODIR})
ki18n_install(po)

feature_summary(WHAT ALL INCLUDE_QUIET_PACKAGES FATAL_ON_MISSING_REQUIRED_PACKAGES)

file(GLOB_RECURSE ALL_CLANG_FORMAT_SOURCE_FILES src/*.cpp src/*.h)

if (UNIX)
    ecm_install_icons(ICONS
            sc-apps-jewels.svg
            DESTINATION ${KDE_INSTALL_ICONDIR} THEME hicolor)
endif()
