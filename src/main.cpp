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

#include "Jewels.h"
#include "jewelsconfig.h"

using namespace Qt::Literals::StringLiterals;

auto main(int argc, char *argv[]) -> int {
  QApplication app(argc, argv);

  auto config = JewelsConfig::self();
  if (!config->isDefaults() &&
      QApplication::arguments().contains("--collect")) {
    jewels::Jewels collector;

    collector.sendData()->future().then([]() { QApplication::quit(); });
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
    qmlRegisterSingletonInstance("dev.imanuel.jewels", 1, 0, "Config", config);
    qmlRegisterSingletonInstance("dev.imanuel.jewels", 1, 0, "App",
                                 new jewels::App());
    qmlRegisterSingletonInstance("dev.imanuel.jewels", 1, 0, "Jewels",
                                 new jewels::Jewels());

    engine.rootContext()->setContextObject(new KLocalizedContext(&engine));
    engine.load(QUrl(u"qrc:/main.qml"_s));

    app.setWindowIcon(QIcon::fromTheme("jewels"));

    if (engine.rootObjects().isEmpty()) {
      return -1;
    }

    return QApplication::exec();
  }
}
