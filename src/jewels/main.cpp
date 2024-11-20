#include <QApplication>
#include <QtGlobal>

#include <QIcon>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QQuickStyle>
#include <QQuickWindow>
#include <QUrl>
#include <QWindow>

#include "app.h"
#include "version-jewels.h"
#include <KAboutData>
#include <KLocalizedContext>
#include <KLocalizedString>

#include "detector_rs/src/qt.cxxqt.h"
#include "jewelsconfig.h"

using namespace Qt::Literals::StringLiterals;

auto main(int argc, char *argv[]) -> int {
  QApplication app(argc, argv);

  auto config = JewelsConfig::self();
  if (!config->isDefaults() &&
      QApplication::arguments().contains("--collect")) {
    jewels::Sender sender;
    sender.sendData(JewelsConfig::host(), JewelsConfig::token());

    return QApplication::exec();
  } else {
    // Default to org.kde.desktop style unless the user forces another style
    if (qEnvironmentVariableIsEmpty("QT_QUICK_CONTROLS_STYLE")) {
      QQuickStyle::setStyle(u"org.kde.desktop"_s);
    }

    KLocalizedString::setApplicationDomain("jewels");
    QCoreApplication::setOrganizationName(u"Imanuel Ulbricht"_s);

    KAboutData aboutData(
        // The program name used internally.
        u"jewels"_s,
        // A displayable program name string.
        i18nc("@title", "Jewels"),
        // The program version string.
        QStringLiteral(JEWELS_VERSION_STRING),
        // Short description of what the app does.
        i18n("Jewels Desktop Client"),
        // The license this code is released under.
        KAboutLicense::MIT,
        // Copyright Statement.
        i18n("(c) %{CURRENT_YEAR}"));
    aboutData.addAuthor(i18nc("@info:credit", "%{AUTHOR}"),
                        i18nc("@info:credit", "Maintainer"), u"%{EMAIL}"_s,
                        u"https://imanuel.dev"_s);
    KAboutData::setApplicationData(aboutData);
    QQmlApplicationEngine engine;

    const QUrl url(
        QStringLiteral("qrc:/qt/qml/cloud/ulbricht/jewels/qml/ui/main.qml"));
    QObject::connect(
        &engine, &QQmlApplicationEngine::objectCreated, &app,
        [url](QObject *obj, const QUrl &objUrl) {
          if (!obj && url == objUrl)
            QCoreApplication::exit(-1);
        },
        Qt::QueuedConnection);
    qmlRegisterSingletonInstance("cloud.ulbricht.jewels", 2, 0, "Config",
                                 config);
    qmlRegisterSingletonInstance("cloud.ulbricht.jewels", 2, 0, "App",
                                 new jewels::App());

    engine.rootContext()->setContextObject(new KLocalizedContext(&engine));
    engine.load(url);

    app.setWindowIcon(QIcon::fromTheme("jewels"));

    return QApplication::exec();
  }
}
